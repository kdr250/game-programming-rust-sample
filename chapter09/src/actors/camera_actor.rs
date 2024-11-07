use core::f32;
use std::{cell::RefCell, rc::Rc};

use sdl2::{
    keyboard::{KeyboardState, Scancode},
    mouse::RelativeMouseState,
};

use crate::{
    components::{
        audio_component::AudioComponent,
        component::{Component, State as ComponentState},
        move_component::{DefaultMoveComponent, MoveComponent},
    },
    math::{self, matrix4::Matrix4, quaternion::Quaternion, vector3::Vector3},
    system::{
        asset_manager::AssetManager, audio_system::AudioSystem, entity_manager::EntityManager,
        renderer::Renderer, sound_event::SoundEvent,
    },
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
    audio_system: Rc<RefCell<AudioSystem>>,
    move_component: Option<Rc<RefCell<DefaultMoveComponent>>>,
    audio_component: Option<Rc<RefCell<AudioComponent>>>,
    foot_step: Option<Rc<RefCell<SoundEvent>>>,
    last_foot_step: f32,
}

impl CameraActor {
    pub fn new(
        asset_manager: Rc<RefCell<AssetManager>>,
        entity_manager: Rc<RefCell<EntityManager>>,
        renderer: Rc<RefCell<Renderer>>,
        audio_system: Rc<RefCell<AudioSystem>>,
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
            audio_system: audio_system.clone(),
            move_component: None,
            audio_component: None,
            foot_step: None,
            last_foot_step: 0.0,
        };

        let result = Rc::new(RefCell::new(this));

        let move_component = DefaultMoveComponent::new(result.clone());
        result.borrow_mut().move_component = Some(move_component);

        let audio_component = AudioComponent::new(result.clone(), audio_system);
        let sound_event = audio_component.borrow_mut().play_event("event:/Footstep");
        sound_event.borrow_mut().set_paused(true);
        result.borrow_mut().audio_component = Some(audio_component);
        result.borrow_mut().foot_step = Some(sound_event);

        entity_manager.borrow_mut().add_actor(result.clone());

        result
    }

    pub fn set_foot_step_surface(&mut self, value: f32) {
        // Pause here because the way I setup the parameter in FMOD
        // changing it will play a footstep
        let foot_step = self.foot_step.clone().unwrap();
        foot_step.borrow_mut().set_paused(true);
        foot_step.borrow_mut().set_parameter("Surface", value);
    }
}

impl Actor for CameraActor {
    fn update_actor(&mut self, delta_time: f32) {
        // Play the footstep if we're moving and haven't recently
        self.last_foot_step -= delta_time;
        if !math::basic::near_zero(
            self.move_component
                .clone()
                .unwrap()
                .borrow()
                .get_forward_speed(),
            0.001,
        ) && self.last_foot_step <= 0.0
        {
            let foot_step = self.foot_step.clone().unwrap();
            foot_step.borrow_mut().set_paused(false);
            foot_step.borrow_mut().restart();
            self.last_foot_step = 0.5;
        }

        // Compute new camera from this actor
        let camera_position = self.position.clone();
        let target = self.position.clone() + self.get_forward() * 100.0;
        let up = Vector3::UNIT_Z;

        let view = Matrix4::create_look_at(&camera_position, &target, &up);

        self.audio_system.borrow_mut().set_listener(&view);
        self.renderer.borrow_mut().set_view_matrix(view);
    }

    fn actor_input(&mut self, key_state: &KeyboardState, _mouse_state: &RelativeMouseState) {
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
