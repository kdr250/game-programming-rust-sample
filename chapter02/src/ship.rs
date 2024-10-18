use std::{cell::RefCell, rc::Rc};

use sdl2::keyboard::{KeyboardState, Scancode};

use crate::{
    actor::{self, Actor, State},
    anim_sprite_component::AnimSpriteComponent,
    component::{Component, State as ComponentState},
    math::Vector2,
    Game,
};

pub struct Ship {
    state: State,
    position: Vector2,
    scale: f32,
    rotation: f32,
    components: Vec<Rc<RefCell<dyn Component>>>,
    game: Rc<RefCell<Game>>,
    right_speed: f32,
    down_speed: f32,
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
            right_speed: 0.0,
            down_speed: 0.0,
        };

        let result = Rc::new(RefCell::new(this));
        let anim_sprite_component = AnimSpriteComponent::new(result.clone(), 100);
        let mut game = game.borrow_mut();
        let anims = vec![
            game.get_texture("Assets/Ship01.png"),
            game.get_texture("Assets/Ship02.png"),
            game.get_texture("Assets/Ship03.png"),
            game.get_texture("Assets/Ship04.png"),
        ];
        anim_sprite_component.borrow_mut().set_anim_textures(anims);

        game.add_actor(result.clone());

        result
    }

    pub fn process_keyboard(&mut self, state: KeyboardState) {
        self.right_speed = 0.0;
        self.down_speed = 0.0;

        if state.is_scancode_pressed(Scancode::D) {
            self.right_speed += 250.0;
        }
        if state.is_scancode_pressed(Scancode::A) {
            self.right_speed -= 250.0;
        }

        if state.is_scancode_pressed(Scancode::S) {
            self.down_speed += 250.0;
        }
        if state.is_scancode_pressed(Scancode::W) {
            self.down_speed -= 250.0;
        }
    }

    pub fn get_right_speed(&self) -> f32 {
        self.right_speed
    }

    pub fn get_down_speed(&self) -> f32 {
        self.down_speed
    }
}

impl Actor for Ship {
    fn update_actor(&mut self, delta_time: f32) {
        let mut position = self.get_position().clone();
        position.x += self.right_speed * delta_time;
        position.y += self.down_speed * delta_time;

        position.x = position.x.clamp(25.0, 500.0);
        position.y = position.y.clamp(25.0, 743.0);

        self.set_position(position);
    }

    actor::impl_getters_setters! {}
    actor::impl_component_operation! {}
}

impl Drop for Ship {
    actor::impl_drop! {}
}
