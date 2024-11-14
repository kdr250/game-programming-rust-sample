use std::{cell::RefCell, rc::Rc};

use sdl2::keyboard::{KeyboardState, Scancode};

use crate::{
    actors::actor::Actor,
    math::{matrix4::Matrix4, quaternion::Quaternion, vector3::Vector3},
};

use super::{
    component::{self, generate_id, Component, State},
    move_component::{self, impl_getters_setters, MoveComponent},
};

pub struct InputComponent {
    id: u32,
    owner: Rc<RefCell<dyn Actor>>,
    update_order: i32,
    state: State,
    angular_speed: f32,
    forward_speed: f32,
    max_forward_speed: f32,
    max_angular_speed: f32,
    forward_key: Scancode,
    back_key: Scancode,
    clockwise_key: Scancode,
    counter_clockwise_key: Scancode,
}

impl InputComponent {
    pub fn new(owner: Rc<RefCell<dyn Actor>>) -> Rc<RefCell<Self>> {
        let this = Self {
            id: generate_id(),
            owner: owner.clone(),
            update_order: 10,
            state: State::Active,
            angular_speed: 0.0,
            forward_speed: 0.0,
            max_forward_speed: 0.0,
            max_angular_speed: 0.0,
            forward_key: Scancode::Escape,
            back_key: Scancode::Escape,
            clockwise_key: Scancode::Escape,
            counter_clockwise_key: Scancode::Escape,
        };

        let result = Rc::new(RefCell::new(this));

        let mut borrowed_onwer = owner.borrow_mut();
        borrowed_onwer.add_component(result.clone());

        result
    }

    /// Getters/setters
    pub fn get_max_forward(&self) -> f32 {
        self.max_forward_speed
    }

    pub fn get_max_angular(&self) -> f32 {
        self.max_angular_speed
    }

    pub fn get_forward_key(&self) -> Scancode {
        self.forward_key
    }

    pub fn get_back_key(&self) -> Scancode {
        self.back_key
    }

    pub fn get_clockwise_key(&self) -> Scancode {
        self.clockwise_key
    }

    pub fn get_counter_clockwise_key(&self) -> Scancode {
        self.counter_clockwise_key
    }

    pub fn set_max_forward_speed(&mut self, speed: f32) {
        self.max_forward_speed = speed;
    }

    pub fn set_max_angular_speed(&mut self, speed: f32) {
        self.max_angular_speed = speed;
    }

    pub fn set_forward_key(&mut self, key: Scancode) {
        self.forward_key = key;
    }

    pub fn set_back_key(&mut self, key: Scancode) {
        self.back_key = key;
    }

    pub fn set_clockwise_key(&mut self, key: Scancode) {
        self.clockwise_key = key;
    }

    pub fn set_counter_clockwise_key(&mut self, key: Scancode) {
        self.counter_clockwise_key = key;
    }
}

impl MoveComponent for InputComponent {
    impl_getters_setters! {}
}

impl Component for InputComponent {
    fn update(
        &mut self,
        delta_time: f32,
        owner_info: &(Vector3, Quaternion, Vector3, Matrix4),
    ) -> (Option<Vector3>, Option<Quaternion>, Option<Vector3>) {
        move_component::update_move_component(self, delta_time, owner_info)
    }

    component::impl_getters_setters! {}

    fn process_input(&mut self, key_state: &KeyboardState) {
        let mut forward_speed = 0.0;
        if key_state.is_scancode_pressed(self.forward_key) {
            forward_speed += self.max_forward_speed;
        }
        if key_state.is_scancode_pressed(self.back_key) {
            forward_speed -= self.max_forward_speed;
        }
        self.set_forward_speed(forward_speed);

        let mut angular_speed = 0.0;
        if key_state.is_scancode_pressed(self.clockwise_key) {
            angular_speed += self.max_angular_speed;
        }
        if key_state.is_scancode_pressed(self.counter_clockwise_key) {
            angular_speed -= self.max_angular_speed;
        }
        self.set_angular_speed(angular_speed);
    }
}
