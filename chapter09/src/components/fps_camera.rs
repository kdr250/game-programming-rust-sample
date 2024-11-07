use core::f32;
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

pub struct FPSCamera {
    id: u32,
    owner: Rc<RefCell<dyn Actor>>,
    update_order: i32,
    state: State,
    renderer: Rc<RefCell<Renderer>>,
    audio_system: Rc<RefCell<AudioSystem>>,
    // Rotation/sec speed of pitch
    pitch_speed: f32,
    // Maximum pitch deviation from forward
    max_pitch: f32,
    // Current pitch
    pitch: f32,
}

impl FPSCamera {
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
            pitch_speed: 0.0,
            max_pitch: f32::consts::PI / 3.0,
            pitch: 0.0,
        };
        let result = Rc::new(RefCell::new(this));
        owner.borrow_mut().add_component(result.clone());
        result
    }

    pub fn get_pitch(&self) -> f32 {
        self.pitch
    }

    pub fn get_pitch_speed(&self) -> f32 {
        self.pitch_speed
    }

    pub fn get_max_pitch(&self) -> f32 {
        self.max_pitch
    }

    pub fn set_pitch_speed(&mut self, speed: f32) {
        self.pitch_speed = speed;
    }

    pub fn set_max_pitch(&mut self, pitch: f32) {
        self.max_pitch = pitch;
    }
}

impl CameraComponent for FPSCamera {
    camera_component::impl_getters! {}
}

impl Component for FPSCamera {
    fn update(
        &mut self,
        delta_time: f32,
        owner_info: &(Vector3, Quaternion, Vector3, Matrix4, Vector3),
    ) -> (Option<Vector3>, Option<Quaternion>) {
        // Camera position is owner position
        let camera_position = owner_info.0.clone();

        // Update pitch based on pitch speed
        self.pitch += self.pitch_speed * delta_time;
        // Clamp pitch to [-max, +max]
        self.pitch = self.pitch.clamp(-self.max_pitch, self.max_pitch);
        // Make a quaternion representing pitch rotation, which is about owner's right vector
        let q = Quaternion::from_axis_angle(&owner_info.4, self.pitch);

        // Rotate owner forward by pitch quaternion
        let view_forward = Vector3::transform(&owner_info.2, &q);

        // Target position 100 units in front of view forward
        let target = camera_position.clone() + view_forward * 100.0;
        let up = Vector3::UNIT_Z;

        // Create look at matrix, set as view
        let view = Matrix4::create_look_at(&camera_position, &target, &up);
        self.set_view_matrix(view);

        (None, None)
    }

    component::impl_getters_setters! {}
}
