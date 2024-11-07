use core::f32;
use std::{cell::RefCell, rc::Rc};

use crate::{
    components::{
        component::{Component, State as ComponentState},
        spline_camera::{Spline, SplineCamera},
    },
    math::{matrix4::Matrix4, quaternion::Quaternion, vector3::Vector3},
    system::{
        asset_manager::AssetManager, audio_system::AudioSystem, entity_manager::EntityManager,
        renderer::Renderer,
    },
};

use super::actor::{self, generate_id, Actor, State};

pub struct SplineActor {
    id: u32,
    state: State,
    world_transform: Matrix4,
    recompute_world_transform: bool,
    position: Vector3,
    scale: f32,
    rotation: Quaternion,
    components: Vec<Rc<RefCell<dyn Component>>>,
    asset_manager: Rc<RefCell<AssetManager>>,
    entity_manager: Rc<RefCell<EntityManager>>,
    audio_system: Rc<RefCell<AudioSystem>>,
    camera_component: Option<Rc<RefCell<SplineCamera>>>,
}

impl SplineActor {
    pub fn new(
        asset_manager: Rc<RefCell<AssetManager>>,
        entity_manager: Rc<RefCell<EntityManager>>,
        audio_system: Rc<RefCell<AudioSystem>>,
        renderer: Rc<RefCell<Renderer>>,
    ) -> Rc<RefCell<Self>> {
        let this = Self {
            id: generate_id(),
            state: State::Active,
            world_transform: Matrix4::new(),
            recompute_world_transform: true,
            position: Vector3::ZERO,
            scale: 1.0,
            rotation: Quaternion::new(),
            components: vec![],
            asset_manager: asset_manager.clone(),
            entity_manager: entity_manager.clone(),
            audio_system: audio_system.clone(),
            camera_component: None,
        };

        let result = Rc::new(RefCell::new(this));

        // Create a spline
        let mut path = Spline::new();
        for i in 0..5 {
            if i % 2 == 0 {
                path.control_points
                    .push(Vector3::new(300.0 * (i + 1) as f32, 300.0, 300.0));
            } else {
                path.control_points
                    .push(Vector3::new(300.0 * (i + 1) as f32, 0.0, 0.0));
            }
        }

        let spline_camera = SplineCamera::new(result.clone(), renderer, audio_system);
        spline_camera.borrow_mut().set_spline(path);
        spline_camera.borrow_mut().set_paused(false);
        result.borrow_mut().camera_component = Some(spline_camera);

        entity_manager.borrow_mut().add_actor(result.clone());

        result
    }

    pub fn restart_spline(&mut self) {
        let camera_component = self.camera_component.as_ref().unwrap();
        camera_component.borrow_mut().restart();
    }
}

impl Actor for SplineActor {
    fn update_actor(&mut self, _delta_time: f32) {}

    actor::impl_getters_setters! {}

    actor::impl_component_operation! {}
}
