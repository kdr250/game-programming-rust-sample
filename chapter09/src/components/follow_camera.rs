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

pub struct FollowCamera {
    id: u32,
    owner: Rc<RefCell<dyn Actor>>,
    update_order: i32,
    state: State,
    renderer: Rc<RefCell<Renderer>>,
    audio_system: Rc<RefCell<AudioSystem>>,
    actual_position: Vector3,
    velocity: Vector3,
    horizontal_distance: f32,
    vertical_distance: f32,
    target_distance: f32,
    spring_constant: f32,
}

impl FollowCamera {
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
            actual_position: Vector3::ZERO,
            velocity: Vector3::ZERO,
            horizontal_distance: 350.0,
            vertical_distance: 150.0,
            target_distance: 100.0,
            spring_constant: 64.0,
        };
        let result = Rc::new(RefCell::new(this));
        owner.borrow_mut().add_component(result.clone());
        result
    }

    pub fn set_horizontal_distance(&mut self, distance: f32) {
        self.horizontal_distance = distance;
    }

    pub fn set_vertical_distance(&mut self, distance: f32) {
        self.vertical_distance = distance;
    }

    pub fn set_target_distance(&mut self, distance: f32) {
        self.target_distance = distance;
    }

    pub fn snap_to_ideal(&mut self) {
        let owner_position = self.owner.borrow().get_position().clone();
        let owner_forward = self.owner.borrow().get_forward().clone();
        let up = Vector3::UNIT_Z;

        // Set actual position to ideal
        self.actual_position =
            self.compute_camera_position(owner_position.clone(), owner_forward.clone(), up.clone());

        // Zero velocity
        self.velocity = Vector3::ZERO;

        // Compute target and view
        let target = owner_position + owner_forward * self.target_distance;
        let view = Matrix4::create_look_at(&self.actual_position, &target, &up);
        self.set_view_matrix(view);
    }

    fn compute_camera_position(
        &self,
        owner_position: Vector3,
        owner_forward: Vector3,
        up: Vector3,
    ) -> Vector3 {
        let mut camera_position = owner_position;
        camera_position -= owner_forward * self.horizontal_distance;
        camera_position += up * self.vertical_distance;
        camera_position
    }
}

impl CameraComponent for FollowCamera {
    camera_component::impl_getters! {}
}

impl Component for FollowCamera {
    fn update(
        &mut self,
        delta_time: f32,
        owner_info: &(Vector3, Quaternion, Vector3, Matrix4, Vector3),
    ) -> (Option<Vector3>, Option<Quaternion>) {
        // Compute dampening from spring constant
        let dampening = 2.0 * self.spring_constant.sqrt();

        // Cmppute ideal position
        let ideal_position = self.compute_camera_position(
            owner_info.0.clone(),
            owner_info.2.clone(),
            Vector3::UNIT_Z,
        );

        // Compute difference between actual and ideal
        let diff = self.actual_position.clone() - ideal_position;

        // Compute acceleration of spring
        let acceleration = diff * -self.spring_constant - self.velocity.clone() * dampening;

        // Update velocity
        self.velocity += acceleration * delta_time;
        // Update actual camera position
        self.actual_position += self.velocity.clone() * delta_time;
        // Target is target dist in front of owning actor
        let target = owner_info.0.clone() + owner_info.2.clone() * self.target_distance;

        // Use actual position here, not ideal
        let view = Matrix4::create_look_at(&self.actual_position, &target, &Vector3::UNIT_Z);
        self.set_view_matrix(view);

        (None, None)
    }

    component::impl_getters_setters! {}
}
