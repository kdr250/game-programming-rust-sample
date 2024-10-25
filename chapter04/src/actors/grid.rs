use std::{cell::RefCell, rc::Rc};

use crate::{
    components::component::{Component, State as ComponentState},
    math::vector2::Vector2,
    system::{entity_manager::EntityManager, texture_manager::TextureManager},
};

use super::{
    actor::{self, generate_id, Actor, State},
    enemy::Enemy,
    tile::{Tile, TileState},
    tower::Tower,
};

pub struct Grid {
    id: u32,
    state: State,
    position: Vector2,
    scale: f32,
    rotation: f32,
    components: Vec<Rc<RefCell<dyn Component>>>,
    texture_manager: Rc<RefCell<TextureManager>>,
    entity_manager: Rc<RefCell<EntityManager>>,
    selected_tile: Option<Rc<RefCell<Tile>>>,
    tiles: Vec<Vec<Rc<RefCell<Tile>>>>,
    next_enemy: f32,
}

impl Grid {
    const NUM_ROW: usize = 7;
    const NUM_COLUMN: usize = 16;
    const START_Y: f32 = 192.0;
    const TILE_SIZE: f32 = 64.0;
    const ENEMY_TIME: f32 = 1.5;

    pub fn new(
        texture_manager: Rc<RefCell<TextureManager>>,
        entity_manager: Rc<RefCell<EntityManager>>,
    ) -> Rc<RefCell<Self>> {
        let mut this = Self {
            id: generate_id(),
            state: State::Active,
            position: Vector2::ZERO,
            scale: 1.0,
            rotation: 0.0,
            components: vec![],
            texture_manager: texture_manager.clone(),
            entity_manager: entity_manager.clone(),
            selected_tile: None,
            tiles: vec![],
            next_enemy: 0.0,
        };

        // Create tiles
        for i in 0..Grid::NUM_ROW {
            let mut temps = vec![];
            for j in 0..Grid::NUM_COLUMN {
                let tile = Tile::new(texture_manager.clone(), entity_manager.clone());
                let x = Grid::TILE_SIZE / 2.0 + j as f32 * Grid::TILE_SIZE;
                let y = Grid::START_Y + i as f32 * Grid::TILE_SIZE;
                tile.borrow_mut().set_position(Vector2::new(x, y));
                temps.push(tile);
            }
            this.tiles.push(temps);
        }

        // Set start/end tiles
        this.get_start_tile()
            .borrow_mut()
            .set_tile_state(TileState::Start);
        this.get_end_tile()
            .borrow_mut()
            .set_tile_state(TileState::Base);

        // Set up adjacency lists
        for i in 0..Grid::NUM_ROW {
            for j in 0..Grid::NUM_COLUMN {
                if i > 0 {
                    this.tiles[i][j]
                        .borrow_mut()
                        .add_adjacent(this.tiles[i - 1][j].clone());
                }
                if i < Grid::NUM_ROW - 1 {
                    this.tiles[i][j]
                        .borrow_mut()
                        .add_adjacent(this.tiles[i + 1][j].clone());
                }
                if j > 0 {
                    this.tiles[i][j]
                        .borrow_mut()
                        .add_adjacent(this.tiles[i][j - 1].clone());
                }
                if j < Grid::NUM_COLUMN - 1 {
                    this.tiles[i][j]
                        .borrow_mut()
                        .add_adjacent(this.tiles[i][j + 1].clone());
                }
            }
        }

        // Find path (in reverse)
        this.find_path(this.get_end_tile().clone(), this.get_start_tile().clone());
        this.update_path_tiles(this.get_start_tile().clone());

        this.next_enemy = Grid::ENEMY_TIME;

        let result = Rc::new(RefCell::new(this));

        entity_manager.borrow_mut().add_actor(result.clone());

        result
    }

    pub fn process_click(&mut self, mut x: i32, mut y: i32) {
        y -= (Grid::START_Y - Grid::TILE_SIZE / 2.0) as i32;

        if x < 0 || y < 0 {
            return;
        }

        x /= Grid::TILE_SIZE as i32;
        y /= Grid::TILE_SIZE as i32;

        if x < Grid::NUM_COLUMN as i32 && y < Grid::NUM_ROW as i32 {
            self.select_tile(y, x);
        }
    }

    pub fn find_path(&self, start: Rc<RefCell<Tile>>, goal: Rc<RefCell<Tile>>) -> bool {
        for i in 0..Grid::NUM_ROW {
            for j in 0..Grid::NUM_COLUMN {
                self.tiles[i][j].borrow_mut().g = 0.0;
                self.tiles[i][j].borrow_mut().in_open_set = false;
                self.tiles[i][j].borrow_mut().in_closed_set = false;
            }
        }

        let mut open_set = vec![];

        let mut current = start;
        current.borrow_mut().in_closed_set = true;

        let goal_pos = goal.borrow().get_position().clone();

        let mut is_first = true;
        while is_first || current.borrow().get_id() != goal.borrow().get_id() {
            for neighbor in current.borrow().adjacent.clone() {
                let mut borrowed_neighbor = neighbor.borrow_mut();
                if borrowed_neighbor.blocked {
                    continue;
                }

                if !borrowed_neighbor.in_closed_set {
                    if !borrowed_neighbor.in_open_set {
                        borrowed_neighbor.parent = Some(current.clone());
                        borrowed_neighbor.h =
                            (borrowed_neighbor.get_position().clone() - goal_pos.clone()).length();
                        borrowed_neighbor.g = current.borrow().g + Grid::TILE_SIZE;
                        borrowed_neighbor.f = borrowed_neighbor.g + borrowed_neighbor.h;
                        borrowed_neighbor.in_open_set = true;
                        open_set.push(neighbor.clone());
                    } else {
                        let new_g = current.borrow().g + Grid::TILE_SIZE;
                        if new_g < borrowed_neighbor.g {
                            borrowed_neighbor.parent = Some(current.clone());
                            borrowed_neighbor.g = new_g;
                            borrowed_neighbor.f = borrowed_neighbor.g + borrowed_neighbor.h;
                        }
                    }
                }
            }

            if open_set.is_empty() {
                break;
            }

            let min = open_set
                .clone()
                .into_iter()
                .min_by(|a, b| a.borrow().f.partial_cmp(&b.borrow().f).unwrap())
                .unwrap();

            current = min.clone();
            open_set.retain(|tile| tile.borrow().get_id() != min.borrow().get_id());
            current.borrow_mut().in_open_set = false;
            current.borrow_mut().in_closed_set = true;
            is_first = false;
        }

        let found = current.borrow().get_id() == goal.borrow().get_id();

        found
    }

    pub fn build_tower(&mut self) {
        if self.selected_tile.is_none() || self.selected_tile.clone().unwrap().borrow().blocked {
            return;
        }

        let selected_tile = self.selected_tile.clone().unwrap();
        selected_tile.borrow_mut().blocked = true;
        if self.find_path(self.get_end_tile().clone(), self.get_start_tile().clone()) {
            let tower = Tower::new(self.texture_manager.clone(), self.entity_manager.clone());
            let position = self.get_selected_tile().borrow().get_position().clone();
            tower.borrow_mut().set_position(position);
        } else {
            // This tower would block the path, so don't allow build
            selected_tile.borrow_mut().blocked = false;
            self.find_path(self.get_end_tile().clone(), self.get_start_tile().clone());
        }
        self.update_path_tiles(self.get_start_tile().clone());
    }

    pub fn get_start_tile(&self) -> &Rc<RefCell<Tile>> {
        &self.tiles[3][0]
    }

    pub fn get_end_tile(&self) -> &Rc<RefCell<Tile>> {
        &self.tiles[3][15]
    }

    pub fn get_selected_tile(&self) -> Rc<RefCell<Tile>> {
        self.selected_tile.clone().unwrap()
    }

    fn select_tile(&mut self, row: i32, column: i32) {
        self.selected_tile = Some(self.tiles[row as usize][column as usize].clone());
    }

    fn update_path_tiles(&mut self, start: Rc<RefCell<Tile>>) {
        for i in 0..Grid::NUM_ROW {
            for j in 0..Grid::NUM_COLUMN {
                if !(i == 3 && j == 0) && !(i == 3 && j == 15) {
                    self.tiles[i][j]
                        .borrow_mut()
                        .set_tile_state(TileState::Default);
                }
            }
        }

        let mut tile = start.borrow().parent.clone().unwrap();

        while tile.borrow().get_id() != self.get_end_tile().borrow().get_id() {
            tile.borrow_mut().set_tile_state(TileState::Path);
            let parent = tile.borrow().parent.clone().unwrap();
            tile = parent;
        }
    }
}

impl Actor for Grid {
    fn update_actor(&mut self, delta_time: f32) {
        self.next_enemy -= delta_time;
        if self.next_enemy <= 0.0 {
            let _ = Enemy::new(
                self.texture_manager.clone(),
                self.entity_manager.clone(),
                self.get_start_tile().clone(),
            );
            self.next_enemy += Grid::ENEMY_TIME;
        }
    }

    actor::impl_getters_setters! {}

    actor::impl_component_operation! {}
}

impl Drop for Grid {
    actor::impl_drop! {}
}
