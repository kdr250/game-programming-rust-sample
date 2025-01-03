use std::{
    cell::RefCell,
    collections::HashMap,
    path::Path,
    rc::Rc,
    sync::atomic::{AtomicU32, Ordering},
};

use anyhow::Result;
use libfmod::{
    ffi::{FMOD_INIT_NORMAL, FMOD_STUDIO_INIT_NORMAL},
    Attributes3d, Bank, Bus, EventDescription, EventInstance, LoadBank, PlaybackState, Studio,
    System, Vector,
};

use crate::math::{matrix4::Matrix4, vector3::Vector3};

use super::{asset_manager::AssetManager, sound_event::SoundEvent};

static ID: AtomicU32 = AtomicU32::new(0);

fn generate_id() -> u32 {
    let id = ID.load(Ordering::SeqCst);
    ID.fetch_add(1, Ordering::SeqCst);
    id
}

pub struct AudioSystem {
    asset_manager: Rc<RefCell<AssetManager>>,
    system: Studio,
    low_level_system: System,
    banks: HashMap<String, Bank>,
    events: HashMap<String, EventDescription>,
    event_instances: HashMap<u32, Rc<RefCell<EventInstance>>>,
    buses: HashMap<String, Bus>,
}

impl AudioSystem {
    pub fn initialize(
        asset_manager: Rc<RefCell<AssetManager>>,
    ) -> Result<Rc<RefCell<AudioSystem>>> {
        let system = Studio::create()?;
        system.initialize(512, FMOD_STUDIO_INIT_NORMAL, FMOD_INIT_NORMAL, None)?;

        let low_level_system = system.get_core_system()?;

        let mut this = Self {
            asset_manager,
            system,
            low_level_system,
            banks: HashMap::new(),
            events: HashMap::new(),
            event_instances: HashMap::new(),
            buses: HashMap::new(),
        };

        this.load_bank("Master Bank.strings.bank")?;
        this.load_bank("Master Bank.bank")?;

        Ok(Rc::new(RefCell::new(this)))
    }

    pub fn load_bank(&mut self, name: &str) -> Result<()> {
        // Prevent double-loading
        if self.banks.contains_key(name) {
            return Ok(());
        }

        // load bank
        let path = Path::new(env!("OUT_DIR"))
            .join("resources")
            .join("Assets")
            .join(name);
        let file_name = path.to_str().unwrap();

        let bank = self.system.load_bank_file(file_name, LoadBank::NORMAL)?;
        self.banks.insert(name.to_string(), bank);
        bank.load_sample_data()?;

        let num_events = bank.get_event_count()?;
        if num_events <= 0 {
            return Ok(());
        }

        let events = bank.get_event_list(num_events)?;
        for event in events {
            let event_name = event.get_path()?;
            self.events.insert(event_name, event);
        }

        Ok(())
    }

    pub fn play_event(&mut self, name: &str) -> SoundEvent {
        let event_description = self.events.get(name).unwrap();
        let event_instance = event_description.create_instance().unwrap();
        event_instance.start().unwrap();
        let id = generate_id();
        let result = Rc::new(RefCell::new(event_instance));
        self.event_instances.insert(id, result.clone());
        SoundEvent::new(id, result)
    }

    pub fn update(&mut self, _delta_time: f32) {
        let mut done = vec![];
        for (id, instance) in self.event_instances.clone() {
            let state = instance.borrow().get_playback_state().unwrap();
            if state == PlaybackState::Stopped {
                instance.borrow_mut().release().unwrap();
                done.push(id);
            }
        }

        for id in done {
            self.event_instances.remove(&id);
        }

        self.system.update().unwrap();
    }

    pub fn set_listener(&mut self, view_matrix: &Matrix4) {
        let mut inverted_view = view_matrix.clone();
        inverted_view.invert();

        let attributes = Attributes3d {
            position: AudioSystem::vector_to_fmod(&inverted_view.get_translation()),
            forward: AudioSystem::vector_to_fmod(&inverted_view.get_z_axis()),
            up: AudioSystem::vector_to_fmod(&inverted_view.get_y_axis()),
            velocity: AudioSystem::vector_to_fmod(&Vector3::ZERO),
        };

        self.system
            .set_listener_attributes(0, attributes, None)
            .unwrap();
    }

    pub fn vector_to_fmod(in_vector: &Vector3) -> Vector {
        Vector::new(in_vector.y, in_vector.z, in_vector.x)
    }

    pub fn get_bus_volume(&self, name: &str) -> f32 {
        self.buses
            .get(name)
            .and_then(|bus| bus.get_volume().ok())
            .and_then(|volumes| Some(volumes.0))
            .unwrap_or(0.0)
    }

    pub fn get_bus_paused(&self, name: &str) -> bool {
        self.buses
            .get(name)
            .and_then(|bus| bus.get_paused().ok())
            .is_some_and(|paused| paused)
    }

    pub fn set_bus_volume(&mut self, name: &str, volume: f32) {
        self.buses
            .get_mut(name)
            .and_then(|bus| bus.set_volume(volume).ok());
    }

    pub fn set_bus_paused(&mut self, name: &str, pause: bool) {
        self.buses
            .get_mut(name)
            .and_then(|bus| bus.set_paused(pause).ok());
    }
}

impl Drop for AudioSystem {
    fn drop(&mut self) {
        let _ = self.system.release();
        let _ = self.low_level_system.release();
    }
}
