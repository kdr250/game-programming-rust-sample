use std::{cell::RefCell, rc::Rc};

use crate::{
    actors::actor::Actor,
    math::{matrix4::Matrix4, quaternion::Quaternion, vector3::Vector3},
    system::{
        audio_system::{self, AudioSystem},
        sound_event::SoundEvent,
    },
};

use super::component::{self, generate_id, Component, State};

pub struct AudioComponent {
    id: u32,
    owner: Rc<RefCell<dyn Actor>>,
    update_order: i32,
    state: State,
    audio_system: Rc<RefCell<AudioSystem>>,
    events_2d: Vec<SoundEvent>,
    events_3d: Vec<SoundEvent>,
    owner_world_transform: Matrix4,
}

impl AudioComponent {
    pub fn new(
        owner: Rc<RefCell<dyn Actor>>,
        audio_system: Rc<RefCell<AudioSystem>>,
    ) -> Rc<RefCell<Self>> {
        let this = Self {
            id: generate_id(),
            owner: owner.clone(),
            update_order: 100,
            state: State::Active,
            audio_system,
            events_2d: vec![],
            events_3d: vec![],
            owner_world_transform: owner.borrow().get_world_transform().clone(),
        };
        let result = Rc::new(RefCell::new(this));
        owner.borrow_mut().add_component(result.clone());
        result
    }

    pub fn play_event(&mut self, name: &str) {
        let mut event = self.audio_system.borrow_mut().play_event(name);
        if event.is_3d() {
            event.set_3d_attributes(self.owner.borrow().get_world_transform());
            self.events_3d.push(event);
        } else {
            self.events_2d.push(event);
        }
    }

    pub fn stop_all_events(&mut self) {
        self.events_2d.iter_mut().for_each(|event| event.stop(true));
        self.events_3d.iter_mut().for_each(|event| event.stop(true));

        self.events_2d.clear();
        self.events_3d.clear();
    }
}

impl Component for AudioComponent {
    fn update(
        &mut self,
        _delta_time: f32,
        owner_info: &(Vector3, Quaternion, Vector3, Matrix4),
    ) -> (Option<Vector3>, Option<Quaternion>) {
        self.owner_world_transform = owner_info.3.clone();
        self.events_2d.retain(|event| event.is_valid());
        self.events_3d.retain(|event| event.is_valid());

        (None, None)
    }

    fn on_update_world_transform(&mut self) {
        let world = &self.owner_world_transform;
        for event in &mut self.events_3d {
            if event.is_valid() {
                event.set_3d_attributes(world);
            }
        }
    }

    component::impl_getters_setters! {}
}

impl Drop for AudioComponent {
    fn drop(&mut self) {
        self.stop_all_events();
    }
}
