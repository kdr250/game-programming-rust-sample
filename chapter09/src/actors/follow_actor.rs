use core::f32;
use std::{cell::RefCell, rc::Rc};

use sdl2::{
    keyboard::{KeyboardState, Scancode},
    mouse::RelativeMouseState,
};

use crate::{
    components::{
        component::{Component, State as ComponentState},
        follow_camera::FollowCamera,
        mesh_component::MeshComponent,
        move_component::{DefaultMoveComponent, MoveComponent},
    },
    math::{self, matrix4::Matrix4, quaternion::Quaternion, vector3::Vector3},
    system::{
        asset_manager::AssetManager, audio_system::AudioSystem, entity_manager::EntityManager,
        renderer::Renderer,
    },
};

use super::actor::{self, generate_id, Actor, State};

pub struct FollowActor {
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
    move_component: Option<Rc<RefCell<DefaultMoveComponent>>>,
    camera_component: Option<Rc<RefCell<FollowCamera>>>,
    mesh_component: Option<Rc<RefCell<MeshComponent>>>,
}

impl FollowActor {
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
            move_component: None,
            camera_component: None,
            mesh_component: None,
        };

        this.set_position(Vector3::new(0.0, 0.0, -100.0));

        let result = Rc::new(RefCell::new(this));

        let move_component = DefaultMoveComponent::new(result.clone());
        result.borrow_mut().move_component = Some(move_component);

        let follow_camera = FollowCamera::new(result.clone(), renderer, audio_system);
        follow_camera.borrow_mut().snap_to_ideal();
        result.borrow_mut().camera_component = Some(follow_camera);

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

impl Actor for FollowActor {
    fn update_actor(&mut self, _delta_time: f32) {}

    fn actor_input(&mut self, key_state: &KeyboardState, _mouse_state: &RelativeMouseState) {
        let mut forward_speed = 0.0;
        let mut angular_speed = 0.0;

        if key_state.is_scancode_pressed(Scancode::W) {
            forward_speed += 400.0;
        }
        if key_state.is_scancode_pressed(Scancode::S) {
            forward_speed -= 400.0;
        }
        if key_state.is_scancode_pressed(Scancode::A) {
            angular_speed -= f32::consts::PI;
        }
        if key_state.is_scancode_pressed(Scancode::D) {
            angular_speed += f32::consts::PI;
        }

        let move_component = self.move_component.clone().unwrap();
        move_component.borrow_mut().set_forward_speed(forward_speed);
        move_component.borrow_mut().set_angular_speed(angular_speed);

        let camera_component = self.camera_component.as_ref().unwrap();
        camera_component.borrow_mut().set_horizontal_distance(350.0);
    }

    actor::impl_getters_setters! {}

    actor::impl_component_operation! {}
}
