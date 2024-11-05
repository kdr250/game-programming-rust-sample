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
        };
        let result = Rc::new(RefCell::new(this));
        owner.borrow_mut().add_component(result.clone());
        result
    }
}

impl CameraComponent for FPSCamera {
    camera_component::impl_getters! {}
}

impl Component for FPSCamera {
    fn update(
        &mut self,
        _delta_time: f32,
        owner_info: &(Vector3, Quaternion, Vector3, Matrix4, Vector3),
    ) -> (Option<Vector3>, Option<Quaternion>) {
        let camera_position = owner_info.0.clone();
        let target = camera_position.clone() + owner_info.2.clone() * 100.0;
        let up = Vector3::UNIT_Z;

        // Create look at matrix, set as view
        let view = Matrix4::create_look_at(&camera_position, &target, &up);
        self.set_view_matrix(view);

        (None, None)
    }

    component::impl_getters_setters! {}
}
