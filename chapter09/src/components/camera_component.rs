use std::{cell::RefCell, rc::Rc};

use crate::{
    actors::actor::Actor,
    math::{matrix4::Matrix4, quaternion::Quaternion, vector3::Vector3},
    system::{audio_system::AudioSystem, renderer::Renderer},
};

use super::component::{self, generate_id, Component, State};

pub trait CameraComponent {
    fn set_view_matrix(&mut self, view: Matrix4) {
        // Pass view matrix to renderer and audio system
        self.get_audio_system().borrow_mut().set_listener(&view);
        self.get_renderer().borrow_mut().set_view_matrix(view);
    }

    fn get_renderer(&self) -> &Rc<RefCell<Renderer>>;

    fn get_audio_system(&self) -> &Rc<RefCell<AudioSystem>>;
}

macro_rules! impl_getters {
    () => {
        fn get_renderer(&self) -> &Rc<RefCell<Renderer>> {
            &self.renderer
        }

        fn get_audio_system(&self) -> &Rc<RefCell<AudioSystem>> {
            &self.audio_system
        }
    };
}

pub(crate) use impl_getters;

pub struct DefaultCameraComponent {
    id: u32,
    owner: Rc<RefCell<dyn Actor>>,
    update_order: i32,
    state: State,
    renderer: Rc<RefCell<Renderer>>,
    audio_system: Rc<RefCell<AudioSystem>>,
}

impl DefaultCameraComponent {
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

impl CameraComponent for DefaultCameraComponent {
    impl_getters! {}
}

impl Component for DefaultCameraComponent {
    fn update(
        &mut self,
        _delta_time: f32,
        _owner_info: &(Vector3, Quaternion, Vector3, Matrix4, Vector3),
    ) -> (Option<Vector3>, Option<Quaternion>) {
        (None, None)
    }

    component::impl_getters_setters! {}
}
