use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use libfmod::{
    ffi::{FMOD_INIT_NORMAL, FMOD_STUDIO_INIT_NORMAL},
    Studio, System,
};

use super::asset_manager::AssetManager;

pub struct AudioSystem {
    asset_manager: Rc<RefCell<AssetManager>>,
    system: Studio,
    low_level_system: System,
}

impl AudioSystem {
    pub fn initialize(
        asset_manager: Rc<RefCell<AssetManager>>,
    ) -> Result<Rc<RefCell<AudioSystem>>> {
        let system = Studio::create()?;
        system.initialize(512, FMOD_STUDIO_INIT_NORMAL, FMOD_INIT_NORMAL, None)?;

        let low_level_system = system.get_core_system()?;

        let this = Self {
            asset_manager,
            system,
            low_level_system,
        };

        Ok(Rc::new(RefCell::new(this)))
    }

    pub fn update(&mut self, _delta_time: f32) {
        self.system.update().unwrap();
    }
}

impl Drop for AudioSystem {
    fn drop(&mut self) {
        let _ = self.system.release();
        let _ = self.low_level_system.release();
    }
}
