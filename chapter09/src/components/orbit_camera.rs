use std::{cell::RefCell, rc::Rc};

use crate::{
    actors::actor::Actor,
    math::{matrix4::Matrix4, quaternion::Quaternion, vector3::Vector3},
    system::{audio_system::AudioSystem, renderer::Renderer},
};

use super::{
    camera_component::{self, CameraComponent},
    component::{self, generate_id, Component, State},
};

pub struct OrbitCamera {
    id: u32,
    owner: Rc<RefCell<dyn Actor>>,
    update_order: i32,
    state: State,
    renderer: Rc<RefCell<Renderer>>,
    audio_system: Rc<RefCell<AudioSystem>>,
    offset: Vector3,
    up: Vector3,
    pitch_speed: f32,
    yaw_speed: f32,
}

impl OrbitCamera {
    pub fn new(
        owner: Rc<RefCell<dyn Actor>>,
        renderer: Rc<RefCell<Renderer>>,
        audio_system: Rc<RefCell<AudioSystem>>,
    ) -> Rc<RefCell<Self>> {
        let this = Self {
            id: generate_id(),
            owner: owner.clone(),
            update_order: 200,
            state: State::Active,
            renderer,
            audio_system,
            offset: Vector3::new(-400.0, 0.0, 0.0),
            up: Vector3::UNIT_Z,
            pitch_speed: 0.0,
            yaw_speed: 0.0,
        };
        let result = Rc::new(RefCell::new(this));
        owner.borrow_mut().add_component(result.clone());
        result
    }

    pub fn get_pitch_speed(&self) -> f32 {
        self.pitch_speed
    }

    pub fn get_yaw_speed(&self) -> f32 {
        self.yaw_speed
    }

    pub fn set_pitch_speed(&mut self, speed: f32) {
        self.pitch_speed = speed;
    }

    pub fn set_yaw_speed(&mut self, speed: f32) {
        self.yaw_speed = speed;
    }
}

impl CameraComponent for OrbitCamera {
    camera_component::impl_getters! {}
}

impl Component for OrbitCamera {
    fn update(
        &mut self,
        delta_time: f32,
        owner_info: &(Vector3, Quaternion, Vector3, Matrix4, Vector3),
    ) -> (Option<Vector3>, Option<Quaternion>) {
        let yaw = Quaternion::from_axis_angle(&Vector3::UNIT_Z, self.yaw_speed * delta_time);
        self.offset = Vector3::transform(&self.offset, &yaw);
        self.up = Vector3::transform(&self.up, &yaw);

        let mut forward = self.offset.clone() * -1.0;
        forward.normalize_mut();
        let mut right = Vector3::cross(&self.up, &forward);
        right.normalize_mut();

        let pitch = Quaternion::from_axis_angle(&right, self.pitch_speed * delta_time);
        self.offset = Vector3::transform(&self.offset, &pitch);
        self.up = Vector3::transform(&self.up, &pitch);

        let target = owner_info.0.clone();
        let camera_position = target.clone() + self.offset.clone();
        let view = Matrix4::create_look_at(&camera_position, &target, &self.up);
        self.set_view_matrix(view);

        (None, None)
    }

    component::impl_getters_setters! {}
}
