use core::f32;
use std::{cell::RefCell, rc::Rc};

use sdl2::keyboard::{KeyboardState, Scancode};

use crate::{
    components::{
        component::{Component, State as ComponentState},
        move_component::{DefaultMoveComponent, MoveComponent},
    },
    math::{matrix4::Matrix4, quaternion::Quaternion, vector3::Vector3},
    system::{asset_manager::AssetManager, entity_manager::EntityManager, renderer::Renderer},
};

use super::actor::{self, generate_id, Actor, State};

pub struct CameraActor {
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
    renderer: Rc<RefCell<Renderer>>,
    move_component: Option<Rc<RefCell<DefaultMoveComponent>>>,
}

impl CameraActor {
    pub fn new(
        asset_manager: Rc<RefCell<AssetManager>>,
        entity_manager: Rc<RefCell<EntityManager>>,
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
            asset_manager,
            entity_manager: entity_manager.clone(),
            renderer,
            move_component: None,
        };

        let result = Rc::new(RefCell::new(this));

        let move_component = DefaultMoveComponent::new(result.clone());
        result.borrow_mut().move_component = Some(move_component);

        entity_manager.borrow_mut().add_actor(result.clone());

        result
    }
}

impl Actor for CameraActor {
    fn update_actor(&mut self, _delta_time: f32) {
        // Compute new camera from this actor
        let camera_position = self.position.clone();
        let target = self.position.clone() + self.get_forward() * 100.0;
        let up = Vector3::UNIT_Z;

        let view = Matrix4::create_look_at(&camera_position, &target, &up);

        self.renderer.borrow_mut().set_view_matrix(view);
    }

    fn actor_input(&mut self, key_state: &KeyboardState) {
        let mut forward_speed = 0.0;
        let mut angular_speed = 0.0;

        if key_state.is_scancode_pressed(Scancode::W) {
            forward_speed += 300.0;
        }
        if key_state.is_scancode_pressed(Scancode::S) {
            forward_speed -= 300.0;
        }
        if key_state.is_scancode_pressed(Scancode::A) {
            angular_speed -= f32::consts::TAU;
        }
        if key_state.is_scancode_pressed(Scancode::D) {
            angular_speed += f32::consts::TAU;
        }

        let move_component = self.move_component.clone().unwrap();
        move_component.borrow_mut().set_forward_speed(forward_speed);
        move_component.borrow_mut().set_angular_speed(angular_speed);
    }

    actor::impl_getters_setters! {}

    actor::impl_component_operation! {}
}
