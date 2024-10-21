use std::{cell::RefCell, rc::Rc};

use crate::{
    actors::actor::Actor,
    math::{self, vector2::Vector2},
};

use super::component::{self, generate_id, Component, State};

pub struct MoveComponent {
    id: u32,
    owner: Rc<RefCell<dyn Actor>>,
    update_order: i32,
    state: State,
    angular_speed: f32,
    forward_speed: f32,
}

impl MoveComponent {
    pub fn new(owner: Rc<RefCell<dyn Actor>>) -> Rc<RefCell<Self>> {
        let this = Self {
            id: generate_id(),
            owner: owner.clone(),
            update_order: 10,
            state: State::Active,
            angular_speed: 0.0,
            forward_speed: 0.0,
        };

        let result = Rc::new(RefCell::new(this));

        let mut borrowed_onwer = owner.borrow_mut();
        borrowed_onwer.add_component(result.clone());

        result
    }

    pub fn get_angular_speed(&self) -> f32 {
        self.angular_speed
    }

    pub fn get_forward_speed(&self) -> f32 {
        self.forward_speed
    }

    pub fn set_angular_speed(&mut self, speed: f32) {
        self.angular_speed = speed;
    }

    pub fn set_forward_speed(&mut self, speed: f32) {
        self.forward_speed = speed;
    }
}

impl Component for MoveComponent {
    fn update(
        &mut self,
        delta_time: f32,
        owner_info: &(Vector2, f32, Vector2),
    ) -> (Option<Vector2>, Option<f32>) {
        let mut result = (None, None);

        if !math::basic::near_zero(self.angular_speed, 0.001) {
            let mut rotation = owner_info.1;
            rotation += self.angular_speed * delta_time;
            result.1 = Some(rotation);
        }

        if !math::basic::near_zero(self.forward_speed, 0.001) {
            let mut position = owner_info.0.clone();
            position += owner_info.2.clone() * self.forward_speed * delta_time;

            if position.x < 0.0 {
                position.x = 1022.0;
            } else if position.x > 1024.0 {
                position.x = 2.0;
            }

            if position.y < 0.0 {
                position.y = 766.0;
            } else if position.y > 768.0 {
                position.y = 2.0;
            }

            result.0 = Some(position);
        }

        result
    }

    component::impl_getters_setters! {}
}
