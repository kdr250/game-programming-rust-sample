use std::{cell::RefCell, rc::Rc};

use crate::{
    components::{
        component::{Component, State as ComponentState},
        sprite_component::{DefaultSpriteComponent, SpriteComponent},
    },
    math::vector2::Vector2,
    system::{entity_manager::EntityManager, texture_manager::TextureManager},
};

use super::actor::{self, generate_id, Actor, State};

#[derive(Debug, PartialEq, Eq)]
pub enum TileState {
    Default,
    Path,
    Start,
    Base,
}

pub struct Tile {
    id: u32,
    state: State,
    position: Vector2,
    scale: f32,
    rotation: f32,
    components: Vec<Rc<RefCell<dyn Component>>>,
    texture_manager: Rc<RefCell<TextureManager>>,
    entity_manager: Rc<RefCell<EntityManager>>,
    // for path finding
    pub adjacent: Vec<Rc<RefCell<Tile>>>,
    pub parent: Option<Rc<RefCell<Tile>>>,
    pub f: f32,
    pub g: f32,
    pub h: f32,
    pub in_open_set: bool,
    pub in_closed_set: bool,
    pub blocked: bool,
    sprite: Option<Rc<RefCell<DefaultSpriteComponent>>>,
    pub tile_state: TileState,
    pub selected: bool,
}

impl Tile {
    pub fn new(
        texture_manager: Rc<RefCell<TextureManager>>,
        entity_manager: Rc<RefCell<EntityManager>>,
    ) -> Rc<RefCell<Self>> {
        let this = Self {
            id: generate_id(),
            state: State::Active,
            position: Vector2::ZERO,
            scale: 1.0,
            rotation: 0.0,
            components: vec![],
            texture_manager,
            entity_manager: entity_manager.clone(),
            adjacent: vec![],
            parent: None,
            f: 0.0,
            g: 0.0,
            h: 0.0,
            in_open_set: false,
            in_closed_set: false,
            blocked: false,
            sprite: None,
            tile_state: TileState::Default,
            selected: false,
        };

        let result = Rc::new(RefCell::new(this));
        result.borrow_mut().sprite = Some(DefaultSpriteComponent::new(result.clone(), 100));
        result.borrow_mut().update_texture();

        entity_manager.borrow_mut().add_actor(result.clone());

        result
    }

    pub fn add_adjacent(&mut self, adjacent: Rc<RefCell<Tile>>) {
        self.adjacent.push(adjacent);
    }

    pub fn set_tile_state(&mut self, state: TileState) {
        self.tile_state = state;
        self.update_texture();
    }

    pub fn get_tile_state(&self) -> &TileState {
        &self.tile_state
    }

    pub fn toggle_select(&mut self) {
        self.selected = !self.selected;
        self.update_texture();
    }

    fn update_texture(&mut self) {
        let text = match self.tile_state {
            TileState::Start => "Assets/TileTan.png",
            TileState::Base => "Assets/TileGreen.png",
            TileState::Path => {
                if self.selected {
                    "Assets/TileGreySelected.png"
                } else {
                    "Assets/TileGrey.png"
                }
            }
            TileState::Default => {
                if self.selected {
                    "Assets/TileBrownSelected.png"
                } else {
                    "Assets/TileBrown.png"
                }
            }
        };

        self.sprite
            .clone()
            .unwrap()
            .borrow_mut()
            .set_texture(self.texture_manager.borrow_mut().get_texture(&text));
    }
}

impl Actor for Tile {
    fn update_actor(&mut self, _delta_time: f32) {}

    actor::impl_getters_setters! {}

    actor::impl_component_operation! {}
}

impl Drop for Tile {
    actor::impl_drop! {}
}
