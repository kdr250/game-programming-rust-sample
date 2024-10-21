use std::{cell::RefCell, rc::Rc};

use sdl2::render::Texture;

use crate::{
    components::{
        circle_component::CircleComponent,
        component::{Component, State as ComponentState},
        move_component::{DefaultMoveComponent, MoveComponent},
        sprite_component::{DefaultSpriteComponent, SpriteComponent},
    },
    math::vector2::Vector2,
    Game,
};

use super::actor::{self, Actor, State};

pub struct Laser {
    state: State,
    position: Vector2,
    scale: f32,
    rotation: f32,
    components: Vec<Rc<RefCell<dyn Component>>>,
    game: Rc<RefCell<Game>>,
    circle: Option<Rc<RefCell<CircleComponent>>>,
    death_timer: f32,
}

impl Laser {
    pub fn new(game: Rc<RefCell<Game>>, texture: Rc<Texture>) -> Rc<RefCell<Self>> {
        let this = Self {
            state: State::Active,
            position: Vector2::ZERO,
            scale: 1.0,
            rotation: 0.0,
            components: vec![],
            game: game.clone(),
            circle: None,
            death_timer: 0.0,
        };

        let result = Rc::new(RefCell::new(this));

        // Create a sprite component
        let sprite_component: Rc<RefCell<dyn SpriteComponent>> =
            DefaultSpriteComponent::new(result.clone(), 100);
        sprite_component.borrow_mut().set_texture(texture);

        // Create a move component, and set a forward speed
        let move_component: Rc<RefCell<dyn MoveComponent>> =
            DefaultMoveComponent::new(result.clone());
        move_component.borrow_mut().set_forward_speed(150.0);

        // Create a circle component (for collision)
        let circle = CircleComponent::new(result.clone());
        circle.borrow_mut().set_radius(11.0);

        result
    }
}

impl Actor for Laser {
    fn update_actor(&mut self, _delta_time: f32) {
        let mut is_dead = false;
        let binding = self.circle.clone().unwrap();
        let circle = binding.borrow();

        for asteroid in self.game.borrow().get_asteroids() {
            let mut borrowed_asteroid = asteroid.borrow_mut();
            if circle.intersect(borrowed_asteroid.get_circle()) {
                is_dead = true;
                borrowed_asteroid.set_state(State::Dead);
                break;
            }
        }

        if is_dead {
            self.set_state(State::Dead);
        }
    }

    actor::impl_getters_setters! {}

    actor::impl_component_operation! {}
}

impl Drop for Laser {
    actor::impl_drop! {}
}
