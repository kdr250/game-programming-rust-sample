use std::{cell::RefCell, collections::HashMap, rc::Rc};

use anyhow::Result;
use sdl2::{keyboard::Scancode, EventPump};

/// The different button states
#[derive(Debug, PartialEq, Eq)]
pub enum ButtonState {
    None,
    Pressed,
    Released,
    Held,
}

/// Helper for keyboard input
pub struct KeyboardState {
    current_state: Vec<bool>,
    previous_state: [bool; Scancode::Num as usize],
}

impl KeyboardState {
    pub fn new(keyboard_state: &sdl2::keyboard::KeyboardState) -> Self {
        // Assign current state
        let scancodes = keyboard_state
            .scancodes()
            .map(|(key, pressed)| (key as i32, pressed))
            .collect::<HashMap<_, _>>();

        let current_state = (0..Scancode::Num as i32)
            .into_iter()
            .map(|key| *scancodes.get(&key).unwrap_or(&false))
            .collect::<Vec<_>>();

        // Clear previous state
        let previous_state = [false; Scancode::Num as usize];

        Self {
            current_state,
            previous_state,
        }
    }

    /// Copy current state to previous
    pub fn copy_current_to_previous(&mut self) {
        self.previous_state.copy_from_slice(&self.current_state);
    }

    pub fn update(&mut self, keyboard_state: &sdl2::keyboard::KeyboardState) {
        let scancodes = keyboard_state
            .scancodes()
            .map(|(key, pressed)| (key as i32, pressed))
            .collect::<HashMap<_, _>>();

        for i in 0..Scancode::Num as i32 {
            self.current_state[i as usize] = *scancodes.get(&i).unwrap_or(&false);
        }
    }

    pub fn get_key_state(&self, key_code: Scancode) -> ButtonState {
        let previous = self.get_previous_value(key_code);
        let current = self.get_key_value(key_code);

        match (previous, current) {
            (false, false) => ButtonState::None,
            (false, true) => ButtonState::Pressed,
            (true, false) => ButtonState::Released,
            (true, true) => ButtonState::Held,
        }
    }

    pub fn get_key_value(&self, key_code: Scancode) -> bool {
        self.current_state[key_code as usize]
    }

    fn get_previous_value(&self, key_code: Scancode) -> bool {
        self.previous_state[key_code as usize]
    }
}

/// Wrapper that contains current state of input
pub struct InputState {
    pub keyboard: KeyboardState,
}

pub struct InputSystem {
    state: InputState,
}

impl InputSystem {
    pub fn initialize(event_pump: &EventPump) -> Result<Rc<RefCell<Self>>> {
        let keyboard = KeyboardState::new(&event_pump.keyboard_state());

        let state = InputState { keyboard };

        let this = Self { state };

        Ok(Rc::new(RefCell::new(this)))
    }

    // Called right before SDL_PollEvents loop
    pub fn prepare_for_update(&mut self) {
        self.state.keyboard.copy_current_to_previous();
    }

    // Called after SDL_PollEvents loop
    pub fn update(&mut self, event_pump: &EventPump) {
        self.state.keyboard.update(&event_pump.keyboard_state());
    }

    pub fn get_state(&self) -> &InputState {
        &self.state
    }
}
