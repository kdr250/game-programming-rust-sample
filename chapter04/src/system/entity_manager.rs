use std::{cell::RefCell, rc::Rc};

use crate::{
    actors::{
        actor::{self, Actor, State as ActorState},
        enemy::Enemy,
        grid::Grid,
    },
    math::{random::Random, vector2::Vector2},
    system::texture_manager::TextureManager,
};

pub struct EntityManager {
    actors: Vec<Rc<RefCell<dyn Actor>>>,
    pending_actors: Vec<Rc<RefCell<dyn Actor>>>,
    updating_actors: bool,
    enemies: Vec<Rc<RefCell<Enemy>>>,
    grid: Option<Rc<RefCell<Grid>>>,
    random: Random,
}

impl EntityManager {
    pub fn new() -> Rc<RefCell<Self>> {
        let this = Self {
            actors: vec![],
            pending_actors: vec![],
            updating_actors: false,
            enemies: vec![],
            grid: None,
            random: Random::new(),
        };

        let result = Rc::new(RefCell::new(this));

        result
    }

    pub fn add_actor(&mut self, actor: Rc<RefCell<dyn Actor>>) {
        if self.updating_actors {
            self.pending_actors.push(actor);
        } else {
            self.actors.push(actor);
        }
    }

    pub fn add_enemy(&mut self, enemy: Rc<RefCell<Enemy>>) {
        self.enemies.push(enemy);
    }

    pub fn flush_actors(&mut self) {
        for pending in self.pending_actors.clone() {
            self.actors.push(pending);
        }
        self.pending_actors.clear();

        self.actors.retain(|actor| {
            if *actor.borrow().get_state() != ActorState::Dead {
                true
            } else {
                actor::remove_actor(actor.clone());
                false
            }
        });

        self.enemies.retain(|enemy| {
            if *enemy.borrow().get_state() != ActorState::Dead {
                true
            } else {
                actor::remove_actor(enemy.clone());
                false
            }
        });
    }

    pub fn load_data(
        this: Rc<RefCell<EntityManager>>,
        texture_manager: Rc<RefCell<TextureManager>>,
    ) {
        let grid = Grid::new(texture_manager, this.clone());
        this.borrow_mut().grid = Some(grid);
    }

    pub fn get_actors(&self) -> &Vec<Rc<RefCell<dyn Actor>>> {
        &self.actors
    }

    pub fn get_enemies(&self) -> &Vec<Rc<RefCell<Enemy>>> {
        &self.enemies
    }

    pub fn get_nearest_enemy(&self, position: &Vector2) -> Option<Rc<RefCell<Enemy>>> {
        if self.enemies.is_empty() {
            return None;
        }

        let mut best = self.enemies[0].clone();

        let mut best_dist_sq =
            (position.clone() - best.borrow().get_position().clone()).length_sqrt();

        for i in 1..self.enemies.len() {
            let new_dist_sq =
                (position.clone() - self.enemies[i].borrow().get_position().clone()).length_sqrt();
            if new_dist_sq < best_dist_sq {
                best_dist_sq = new_dist_sq;
                best = self.enemies[i].clone();
            }
        }

        Some(best)
    }

    pub fn get_grid(&self) -> Rc<RefCell<Grid>> {
        self.grid.clone().unwrap()
    }

    pub fn get_random(&mut self) -> &mut Random {
        &mut self.random
    }

    pub fn set_updating_actors(&mut self, updating_actors: bool) {
        self.updating_actors = updating_actors;
    }
}
