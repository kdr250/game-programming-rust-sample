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
    Game,
};

pub struct Ship {
    state: State,
    position: Vector2,
    scale: f32,
    rotation: f32,
    components: Vec<Rc<RefCell<dyn Component>>>,
    game: Rc<RefCell<Game>>,
}

impl Ship {
    pub fn new(game: Rc<RefCell<Game>>) -> Rc<RefCell<Self>> {
        let this = Self {
            state: State::Active,
            position: Vector2::ZERO,
            scale: 1.0,
            rotation: 0.0,
            components: vec![],
            game: game.clone(),
        };

        let result = Rc::new(RefCell::new(this));

        let sprite_component = DefaultSpriteComponent::new(result.clone(), 150);
        let mut game = game.borrow_mut();
        sprite_component
            .borrow_mut()
            .set_texture(game.get_texture("Assets/Ship.png"));

        let input_component = InputComponent::new(result.clone());
        let mut borrowed_input = input_component.borrow_mut();
        borrowed_input.set_forward_key(Scancode::W);
        borrowed_input.set_back_key(Scancode::S);
        borrowed_input.set_clockwise_key(Scancode::A);
        borrowed_input.set_counter_clockwise_key(Scancode::D);
        borrowed_input.set_max_forward_speed(300.0);
        borrowed_input.set_max_angular_speed(f32::consts::TAU);

        game.add_actor(result.clone());

        result
    }
}

impl Actor for Ship {
    fn update_actor(&mut self, delta_time: f32) {
        // TODO: Not yet implemented
    }

    fn actor_input(&mut self, _key_state: &KeyboardState) {
        // TODO: Not yet implemented
    }

    actor::impl_getters_setters! {}
    actor::impl_component_operation! {}
}

impl Drop for Ship {
    actor::impl_drop! {}
}
