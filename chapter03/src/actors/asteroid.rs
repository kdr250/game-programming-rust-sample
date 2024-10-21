use core::f32;
use std::{cell::RefCell, rc::Rc};

use crate::{
    components::{
        component::{Component, State as ComponentState},
        move_component::{DefaultMoveComponent, MoveComponent},
        sprite_component::{DefaultSpriteComponent, SpriteComponent},
    },
    math::vector2::Vector2,
    Game,
};

use super::actor::{self, Actor, State};

pub struct Asteroid {
    state: State,
    position: Vector2,
    scale: f32,
    rotation: f32,
    components: Vec<Rc<RefCell<dyn Component>>>,
    game: Rc<RefCell<Game>>,
    // TODO: add circle component
}

impl Asteroid {
    pub fn new(game: Rc<RefCell<Game>>) -> Rc<RefCell<Self>> {
        let mut this = Self {
            state: State::Active,
            position: Vector2::ZERO,
            scale: 1.0,
            rotation: 0.0,
            components: vec![],
            game: game.clone(),
        };

        // Initialize to random position/orientation
        {
            let mut borrowed_game = game.borrow_mut();
            let random = borrowed_game.get_random();
            let random_position = random.get_vector2(Vector2::ZERO, Vector2::new(1024.0, 768.0));
            let random_rotation = random.get_float_range(0.0, f32::consts::TAU);
            this.set_position(random_position);
            this.set_rotation(random_rotation);
        }

        let result = Rc::new(RefCell::new(this));
        game.borrow_mut().add_actor(result.clone());

        // Create a sprite component
        let sprite_component = DefaultSpriteComponent::new(result.clone(), 100);
        sprite_component
            .borrow_mut()
            .set_texture(game.borrow_mut().get_texture("Assets/Asteroid.png"));

        // Create a move component, and set a forward speed
        let move_component: Rc<RefCell<dyn MoveComponent>> =
            DefaultMoveComponent::new(result.clone());
        move_component.borrow_mut().set_forward_speed(150.0);

        // TODO: add circle component

        // TODO: add asteroid to game

        result
    }
}

impl Actor for Asteroid {
    fn update_actor(&mut self, _delta_time: f32) {}

    actor::impl_getters_setters! {}

    actor::impl_component_operation! {}
}

impl Drop for Asteroid {
    actor::impl_drop! {}
}
