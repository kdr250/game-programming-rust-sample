use core::f32;
use std::{cell::RefCell, rc::Rc};

use sdl2::{
    keyboard::{KeyboardState, Scancode},
    mouse::{MouseButton, RelativeMouseState},
};

use crate::{
    components::{
        audio_component::AudioComponent,
        component::{Component, State as ComponentState},
        fps_camera::FPSCamera,
        mesh_component::MeshComponent,
        move_component::{DefaultMoveComponent, MoveComponent},
        orbit_camera::OrbitCamera,
    },
    math::{self, matrix4::Matrix4, quaternion::Quaternion, vector3::Vector3},
    system::{
        asset_manager::AssetManager, audio_system::AudioSystem, entity_manager::EntityManager,
        renderer::Renderer, sound_event::SoundEvent,
    },
};

use super::actor::{self, generate_id, Actor, DefaultActor, State};

pub struct OrbitActor {
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
    camera_component: Option<Rc<RefCell<OrbitCamera>>>,
    mesh_component: Option<Rc<RefCell<MeshComponent>>>,
}

impl OrbitActor {
    pub fn new(
        asset_manager: Rc<RefCell<AssetManager>>,
        entity_manager: Rc<RefCell<EntityManager>>,
        audio_system: Rc<RefCell<AudioSystem>>,
        renderer: Rc<RefCell<Renderer>>,
    ) -> Rc<RefCell<Self>> {
        let mut this = Self {
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
            mesh_component: None,
        };

        this.set_position(Vector3::new(0.0, 0.0, -100.0));

        let result = Rc::new(RefCell::new(this));

        let orbit_camera = OrbitCamera::new(result.clone(), renderer, audio_system);
        result.borrow_mut().camera_component = Some(orbit_camera);

        let mesh_component = MeshComponent::new(result.clone());
        let mesh = asset_manager.borrow_mut().get_mesh("RacingCar.gpmesh");
        mesh_component.borrow_mut().set_mesh(mesh);
        result.borrow_mut().mesh_component = Some(mesh_component);

        entity_manager.borrow_mut().add_actor(result.clone());

        result
    }

    pub fn set_visible(&mut self, visible: bool) {
        let mesh_component = self.mesh_component.as_ref().unwrap();
        mesh_component.borrow_mut().set_visible(visible);
    }
}

impl Actor for OrbitActor {
    fn update_actor(&mut self, _delta_time: f32) {}

    fn actor_input(&mut self, _key_state: &KeyboardState, mouse_state: &RelativeMouseState) {
        let x = mouse_state.x();
        let y = mouse_state.y();

        if mouse_state.is_mouse_button_pressed(MouseButton::Right) {
            let max_mouse_speed = 500.0;
            let max_orbit_speed = f32::consts::PI * 8.0;

            let mut yaw_speed = 0.0;
            if x != 0 {
                // Convert to ~[-1.0, 1.0]
                yaw_speed = x as f32 / max_mouse_speed;
                // Multiply by rotation/sec
                yaw_speed *= max_orbit_speed;
            }

            let camera_component = self.camera_component.as_ref().unwrap();
            camera_component.borrow_mut().set_yaw_speed(-yaw_speed);

            let mut pitch_speed = 0.0;
            if y != 0 {
                // Convert to ~[-1.0, 1.0]
                pitch_speed = y as f32 / max_mouse_speed;
                // Multiply by rotation/sec
                pitch_speed *= max_orbit_speed;
            }
            camera_component.borrow_mut().set_pitch_speed(pitch_speed);
        }
    }

    actor::impl_getters_setters! {}

    actor::impl_component_operation! {}
}
