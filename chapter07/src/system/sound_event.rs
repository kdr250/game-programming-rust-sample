use std::{cell::RefCell, rc::Rc};

use libfmod::{
    ffi::{FMOD_STUDIO_STOP_ALLOWFADEOUT, FMOD_STUDIO_STOP_IMMEDIATE},
    Attributes3d, EventInstance, PlaybackState, StopMode,
};

use crate::math::{matrix4::Matrix4, vector3::Vector3};

use super::audio_system::AudioSystem;

pub struct SoundEvent {
    id: u32,
    event_instance: Rc<RefCell<EventInstance>>,
}

impl SoundEvent {
    pub fn new(id: u32, event_instance: Rc<RefCell<EventInstance>>) -> Self {
        Self { id, event_instance }
    }

    pub fn is_valid(&self) -> bool {
        let state = self.event_instance.borrow().get_playback_state().unwrap();
        state != PlaybackState::Stopped
    }

    pub fn restart(&mut self) {
        self.event_instance.borrow_mut().start().unwrap();
    }

    pub fn stop(&mut self, allow_fade_out: bool) {
        let mode = if allow_fade_out {
            StopMode::AllowFadeout
        } else {
            StopMode::Immediate
        };
        self.event_instance.borrow_mut().stop(mode).unwrap();
    }

    pub fn set_paused(&mut self, pause: bool) {
        self.event_instance.borrow_mut().set_paused(pause).unwrap();
    }

    pub fn set_volume(&mut self, value: f32) {
        self.event_instance.borrow_mut().set_volume(value).unwrap();
    }

    pub fn set_pitch(&mut self, value: f32) {
        self.event_instance.borrow_mut().set_pitch(value).unwrap();
    }

    pub fn set_parameter(&mut self, name: &str, value: f32) {
        self.event_instance
            .borrow_mut()
            .set_parameter_by_name(name, value, false)
            .unwrap();
    }

    pub fn get_paused(&self) -> bool {
        self.event_instance.borrow().get_paused().unwrap()
    }

    pub fn get_volume(&self) -> f32 {
        self.event_instance.borrow().get_volume().unwrap().0
    }

    pub fn get_pitch(&self) -> f32 {
        self.event_instance.borrow().get_pitch().unwrap().0
    }

    pub fn get_parameter(&self, name: &str) -> f32 {
        self.event_instance
            .borrow()
            .get_parameter_by_name(name)
            .unwrap()
            .0
    }

    pub fn is_3d(&self) -> bool {
        self.event_instance
            .borrow()
            .get_description()
            .and_then(|description| description.is_3d())
            .is_ok_and(|is_3d| is_3d)
    }

    pub fn set_3d_attributes(&mut self, world_trans: &Matrix4) {
        let attributes = Attributes3d {
            position: AudioSystem::vector_to_fmod(&world_trans.get_translation()),
            forward: AudioSystem::vector_to_fmod(&world_trans.get_x_axis()),
            up: AudioSystem::vector_to_fmod(&world_trans.get_z_axis()),
            velocity: AudioSystem::vector_to_fmod(&Vector3::ZERO),
        };

        self.event_instance
            .borrow_mut()
            .set_3d_attributes(attributes)
            .unwrap();
    }
}
