use core::f32;
use std::{cell::RefCell, rc::Rc};

use sdl2::keyboard::{KeyboardState, Scancode};

use crate::{
    actors::actor::{self, Actor, State},
    components::{
        component::{Component, State as ComponentState},
        input_component::InputComponent,
        sprite_component::{DefaultSpriteComponent, SpriteComponent},
    },
    math::vector2::Vector2,
    system::{entity_manager::EntityManager, texture_manager::TextureManager},
};

use super::laser::Laser;

pub struct Ship {
    state: State,
    position: Vector2,
    scale: f32,
    rotation: f32,
    components: Vec<Rc<RefCell<dyn Component>>>,
    texture_manager: Rc<RefCell<TextureManager>>,
    entity_manager: Rc<RefCell<EntityManager>>,
    laser_cooldown: f32,
}

impl Ship {
    pub fn new(
        texture_manager: Rc<RefCell<TextureManager>>,
        entity_manager: Rc<RefCell<EntityManager>>,
    ) -> Rc<RefCell<Self>> {
        let this = Self {
            state: State::Active,
            position: Vector2::ZERO,
            scale: 1.0,
            rotation: 0.0,
            components: vec![],
            texture_manager: texture_manager.clone(),
            entity_manager: entity_manager.clone(),
            laser_cooldown: 0.0,
        };

        let result = Rc::new(RefCell::new(this));

        let sprite_component = DefaultSpriteComponent::new(result.clone(), 150);
        let texture = texture_manager.borrow_mut().get_texture("Assets/Ship.png");
        sprite_component.borrow_mut().set_texture(texture);

        let input_component = InputComponent::new(result.clone());
        let mut borrowed_input = input_component.borrow_mut();
        borrowed_input.set_forward_key(Scancode::W);
        borrowed_input.set_back_key(Scancode::S);
        borrowed_input.set_clockwise_key(Scancode::A);
        borrowed_input.set_counter_clockwise_key(Scancode::D);
        borrowed_input.set_max_forward_speed(300.0);
        borrowed_input.set_max_angular_speed(f32::consts::TAU);

        entity_manager.borrow_mut().add_actor(result.clone());

        result
    }
}

impl Actor for Ship {
    fn update_actor(&mut self, delta_time: f32) {
        self.laser_cooldown -= delta_time;
    }

    fn actor_input(&mut self, key_state: &KeyboardState) {
        if key_state.is_scancode_pressed(Scancode::Space) && self.laser_cooldown <= 0.0 {
            let laser = Laser::new(self.texture_manager.clone(), self.entity_manager.clone());
            let mut borrowed_laser = laser.borrow_mut();
            borrowed_laser.set_position(self.position.clone());
            borrowed_laser.set_rotation(self.rotation);

            self.laser_cooldown = 0.5;
        }
    }

    actor::impl_getters_setters! {}
    actor::impl_component_operation! {}
}

impl Drop for Ship {
    actor::impl_drop! {}
}