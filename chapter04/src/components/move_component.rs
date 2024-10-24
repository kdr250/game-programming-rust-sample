use std::{cell::RefCell, rc::Rc};

use crate::{
    actors::actor::Actor,
    math::{self, vector2::Vector2},
};

use super::component::{self, generate_id, Component, State};

pub trait MoveComponent: Component {
    fn get_angular_speed(&self) -> f32;

    fn get_forward_speed(&self) -> f32;

    fn set_angular_speed(&mut self, speed: f32);

    fn set_forward_speed(&mut self, speed: f32);
}

macro_rules! impl_getters_setters {
    () => {
        fn get_angular_speed(&self) -> f32 {
            self.angular_speed
        }

        fn get_forward_speed(&self) -> f32 {
            self.forward_speed
        }

        fn set_angular_speed(&mut self, speed: f32) {
            self.angular_speed = speed;
        }

        fn set_forward_speed(&mut self, speed: f32) {
            self.forward_speed = speed;
        }
    };
}

pub(crate) use impl_getters_setters;

pub fn update_move_component(
    move_component: &dyn MoveComponent,
    delta_time: f32,
    owner_info: &(Vector2, f32, Vector2),
    mut result: (Option<Vector2>, Option<f32>),
) -> (Option<Vector2>, Option<f32>) {
    if !math::basic::near_zero(move_component.get_angular_speed(), 0.001) {
        let temp_rotation = result.1.unwrap_or(0.0);
        let mut rotation = owner_info.1 + temp_rotation;
        rotation += move_component.get_angular_speed() * delta_time;
        result.1 = Some(rotation);
    }

    if !math::basic::near_zero(move_component.get_forward_speed(), 0.001) {
        let temp_position = result.0.clone().unwrap_or(Vector2::ZERO);
        let mut position = owner_info.0.clone() + temp_position;
        position += owner_info.2.clone() * move_component.get_forward_speed() * delta_time;

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

pub struct DefaultMoveComponent {
    id: u32,
    owner: Rc<RefCell<dyn Actor>>,
    update_order: i32,
    state: State,
    angular_speed: f32,
    forward_speed: f32,
}

impl DefaultMoveComponent {
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
}

impl MoveComponent for DefaultMoveComponent {
    impl_getters_setters! {}
}

impl Component for DefaultMoveComponent {
    fn update(
        &mut self,
        delta_time: f32,
        owner_info: &(Vector2, f32, Vector2),
    ) -> (Option<Vector2>, Option<f32>) {
        update_move_component(self, delta_time, owner_info, (None, None))
    }

    component::impl_getters_setters! {}
}
