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
        sound_event::SoundEvent,
    },
};

use super::actor::{self, generate_id, Actor, State};

pub struct FPSActor {
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
    audio_component: Option<Rc<RefCell<AudioComponent>>>,
    foot_step: Option<Rc<RefCell<SoundEvent>>>,
    last_foot_step: f32,
}

impl FPSActor {
    pub fn new(
        asset_manager: Rc<RefCell<AssetManager>>,
        entity_manager: Rc<RefCell<EntityManager>>,
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

        // TODO: FPS Camera

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

    pub fn set_visible(&mut self, visible: bool) {
        // TODO: Not yet implemented
    }
}

impl Actor for FPSActor {
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

        // TODO: Update position of FPS model
    }

    fn actor_input(&mut self, key_state: &KeyboardState, mouse_state: &RelativeMouseState) {
        let mut forward_speed = 0.0;
        let mut strafe_speed = 0.0;

        if key_state.is_scancode_pressed(Scancode::W) {
            forward_speed += 400.0;
        }
        if key_state.is_scancode_pressed(Scancode::S) {
            forward_speed -= 400.0;
        }
        if key_state.is_scancode_pressed(Scancode::A) {
            strafe_speed -= 400.0;
        }
        if key_state.is_scancode_pressed(Scancode::D) {
            strafe_speed += 400.0;
        }

        let move_component = self.move_component.clone().unwrap();
        move_component.borrow_mut().set_forward_speed(forward_speed);
        move_component.borrow_mut().set_strafe_speed(strafe_speed);

        // Mouse movement
        // Get relative movement from SDL
        let x = mouse_state.x();

        // Assume mouse movement is usually between -500 and +500
        let max_mouse_speed = 500.0;

        // Rotation/sec at maximum speed
        let max_angular_speed = f32::consts::PI * 8.0;

        let mut angular_speed = 0.0;
        if x != 0 {
            // Convert to ~[-1.0, 1.0]
            angular_speed = x as f32 / max_mouse_speed;
            // Multiply by rotation/sec
            angular_speed *= max_angular_speed;
        }
        move_component.borrow_mut().set_angular_speed(angular_speed);
    }

    actor::impl_getters_setters! {}

    actor::impl_component_operation! {}
}
