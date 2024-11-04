use std::{cell::RefCell, rc::Rc};

use anyhow::Result;

// The different button states
pub enum ButtonState {
    None,
    Pressed,
    Released,
    Held,
}

// TODO
pub struct KeyboardState;

pub struct InputState {
    keyboard: KeyboardState,
}

pub struct InputSystem {
    state: InputState,
}

impl InputSystem {
    pub fn initialize() -> Result<Rc<RefCell<Self>>> {
        unimplemented!()
    }

    // Called right before SDL_PollEvents loop
    pub fn prepare_for_update(&mut self) {
        unimplemented!()
    }

    // Called after SDL_PollEvents loop
    pub fn update(&mut self) {
        unimplemented!()
    }

    pub fn get_state(&self) -> &InputState {
        &self.state
    }
}
