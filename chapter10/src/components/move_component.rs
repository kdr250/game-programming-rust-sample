use std::{cell::RefCell, rc::Rc};

use crate::{
    actors::actor::Actor,
    math::{self, matrix4::Matrix4, quaternion::Quaternion, vector3::Vector3},
};

use super::component::{self, generate_id, Component, State};

pub trait MoveComponent: Component {
    fn get_angular_speed(&self) -> f32;

    fn get_forward_speed(&self) -> f32;

    fn get_strafe_speed(&self) -> f32;

    fn set_angular_speed(&mut self, speed: f32);

    fn set_forward_speed(&mut self, speed: f32);

    fn set_strafe_speed(&mut self, speed: f32);
}

macro_rules! impl_getters_setters {
    () => {
        fn get_angular_speed(&self) -> f32 {
            self.angular_speed
        }

        fn get_forward_speed(&self) -> f32 {
            self.forward_speed
        }

        fn get_strafe_speed(&self) -> f32 {
            self.strafe_speed
        }

        fn set_angular_speed(&mut self, speed: f32) {
            self.angular_speed = speed;
        }

        fn set_forward_speed(&mut self, speed: f32) {
            self.forward_speed = speed;
        }

        fn set_strafe_speed(&mut self, speed: f32) {
            self.strafe_speed = speed;
        }
    };
}

pub(crate) use impl_getters_setters;

pub fn update_move_component(
    move_component: &dyn MoveComponent,
    delta_time: f32,
    owner_info: &(Vector3, Quaternion, Vector3, Matrix4, Vector3),
) -> (Option<Vector3>, Option<Quaternion>, Option<Vector3>) {
    let mut result = (None, None, None);

    if !math::basic::near_zero(move_component.get_angular_speed(), 0.001) {
        let mut rotation = owner_info.1.clone();
        let angle = move_component.get_angular_speed() * delta_time;

        let increment = Quaternion::from_axis_angle(&Vector3::UNIT_Z, angle);

        rotation = Quaternion::concatenate(&rotation, &increment);
        result.1 = Some(rotation);
    }

    if !math::basic::near_zero(move_component.get_forward_speed(), 0.001)
        || !math::basic::near_zero(move_component.get_strafe_speed(), 0.001)
    {
        let mut position = owner_info.0.clone();
        position += owner_info.2.clone() * move_component.get_forward_speed() * delta_time;
        position += owner_info.4.clone() * move_component.get_strafe_speed() * delta_time;
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
    strafe_speed: f32,
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
            strafe_speed: 0.0,
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
        owner_info: &(Vector3, Quaternion, Vector3, Matrix4, Vector3),
    ) -> (Option<Vector3>, Option<Quaternion>, Option<Vector3>) {
        update_move_component(self, delta_time, owner_info)
    }

    component::impl_getters_setters! {}
}
