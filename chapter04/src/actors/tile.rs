use std::{cell::RefCell, rc::Rc};

use crate::{
    components::{
        component::{Component, State as ComponentState},
        sprite_component::DefaultSpriteComponent,
    },
    math::vector2::Vector2,
    system::{entity_manager::EntityManager, texture_manager::TextureManager},
};

use super::actor::{self, Actor, State};

#[derive(Debug, PartialEq, Eq)]
pub enum TileState {
    Default,
    Path,
    Start,
    Base,
}

pub struct Tile {
    state: State,
    position: Vector2,
    scale: f32,
    rotation: f32,
    components: Vec<Rc<RefCell<dyn Component>>>,
    texture_manager: Rc<RefCell<TextureManager>>,
    entity_manager: Rc<RefCell<EntityManager>>,
    // for path finding
    adjacent: Vec<Rc<RefCell<Tile>>>,
    parent: Option<Rc<RefCell<Tile>>>,
    f: f32,
    g: f32,
    h: f32,
    in_open_set: bool,
    in_closed_set: bool,
    blocked: bool,
    sprite: Option<Rc<RefCell<DefaultSpriteComponent>>>,
    tile_state: TileState,
    selected: bool,
}

impl Tile {
    pub fn new(
        texture_manager: Rc<RefCell<TextureManager>>,
        entity_manager: Rc<RefCell<EntityManager>>,
    ) -> Rc<RefCell<Self>> {
        let mut this = Self {
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

        entity_manager.borrow_mut().add_actor(result.clone());

        result
    }

    pub fn set_tile_state(&mut self, state: TileState) {
        unimplemented!()
    }

    pub fn get_tile_state(&self) -> &TileState {
        &self.tile_state
    }

    pub fn toggle_select(&mut self) {
        unimplemented!()
    }

    pub fn get_parent(&self) -> &Option<Rc<RefCell<Tile>>> {
        &self.parent
    }

    fn update_texture(&mut self) {
        unimplemented!()
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
