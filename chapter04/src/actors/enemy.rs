use std::{cell::RefCell, rc::Rc};

use crate::{
    components::{
        circle_component::CircleComponent,
        component::{Component, State as ComponentState},
        move_component::MoveComponent,
        nav_component::NavComponent,
        sprite_component::{DefaultSpriteComponent, SpriteComponent},
    },
    math::{self, vector2::Vector2},
    system::{entity_manager::EntityManager, texture_manager::TextureManager},
};

use super::{
    actor::{self, generate_id, Actor, State},
    tile::Tile,
};

pub struct Enemy {
    id: u32,
    state: State,
    position: Vector2,
    scale: f32,
    rotation: f32,
    components: Vec<Rc<RefCell<dyn Component>>>,
    texture_manager: Rc<RefCell<TextureManager>>,
    entity_manager: Rc<RefCell<EntityManager>>,
    circle: Option<Rc<RefCell<CircleComponent>>>,
}

impl Enemy {
    pub fn new(
        texture_manager: Rc<RefCell<TextureManager>>,
        entity_manager: Rc<RefCell<EntityManager>>,
        start_tile: Rc<RefCell<Tile>>,
    ) -> Rc<RefCell<Self>> {
        let this = Self {
            id: generate_id(),
            state: State::Active,
            position: Vector2::ZERO,
            scale: 1.0,
            rotation: 0.0,
            components: vec![],
            texture_manager: texture_manager.clone(),
            entity_manager: entity_manager.clone(),
            circle: None,
        };

        let result = Rc::new(RefCell::new(this));

        let sprite_component = DefaultSpriteComponent::new(result.clone(), 100);
        let texture = texture_manager
            .borrow_mut()
            .get_texture("Assets/Airplane.png");
        sprite_component.borrow_mut().set_texture(texture);

        let position = start_tile.borrow().get_position().clone();
        result.borrow_mut().set_position(position);

        let nav_component = NavComponent::new(result.clone(), 10);
        nav_component.borrow_mut().set_forward_speed(150.0);
        nav_component.borrow_mut().start_path(start_tile.clone());

        let circle_component = CircleComponent::new(result.clone());
        circle_component.borrow_mut().set_radius(25.0);
        result.borrow_mut().circle = Some(circle_component);

        entity_manager.borrow_mut().add_actor(result.clone());
        entity_manager.borrow_mut().add_enemy(result.clone());

        result
    }

    pub fn get_circle(&self) -> Rc<RefCell<CircleComponent>> {
        self.circle.clone().unwrap()
    }
}

impl Actor for Enemy {
    fn update_actor(&mut self, _delta_time: f32) {
        let grid = self.entity_manager.borrow().get_grid();
        let binding = grid.borrow();
        let end_tile = binding.get_end_tile();
        let diff = self.get_position().clone() - end_tile.borrow().get_position().clone();
        if math::basic::near_zero(diff.length(), 10.0) {
            self.set_state(State::Dead);
        }
    }

    actor::impl_getters_setters! {}

    actor::impl_component_operation! {}
}

impl Drop for Enemy {
    actor::impl_drop! {}
}
