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
    horizontal_distance: f32,
    vertical_distance: f32,
    target_distance: f32,
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
            horizontal_distance: 350.0,
            vertical_distance: 150.0,
            target_distance: 100.0,
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

    fn compute_camera_position(
        &mut self,
        position: Vector3,
        forward: Vector3,
        up: Vector3,
    ) -> Vector3 {
        let mut camera_position = position;
        camera_position -= forward * self.horizontal_distance;
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
        _delta_time: f32,
        owner_info: &(Vector3, Quaternion, Vector3, Matrix4, Vector3),
    ) -> (Option<Vector3>, Option<Quaternion>) {
        let target = owner_info.0.clone() + owner_info.2.clone() * self.target_distance;
        let camera_position = self.compute_camera_position(
            owner_info.0.clone(),
            owner_info.2.clone(),
            Vector3::UNIT_Z,
        );
        let view = Matrix4::create_look_at(&camera_position, &target, &Vector3::UNIT_Z);
        self.set_view_matrix(view);

        (None, None)
    }

    component::impl_getters_setters! {}
}
