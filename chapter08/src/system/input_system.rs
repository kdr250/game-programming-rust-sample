use std::{cell::RefCell, collections::HashMap, rc::Rc};

use anyhow::Result;
use sdl2::{keyboard::Scancode, mouse::MouseButton, EventPump};

use crate::math::vector2::Vector2;

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
    pub fn new() -> Self {
        Self {
            current_state: vec![false; Scancode::Num as usize],
            previous_state: [false; Scancode::Num as usize],
        }
    }

    /// Copy current state to previous
    pub fn copy_current_to_previous(&mut self) {
        self.previous_state.copy_from_slice(&self.current_state);
    }

    pub fn update(&mut self, keyboard_state: &sdl2::keyboard::KeyboardState) {
        let current_state = (0..Scancode::Num as i32)
            .into_iter()
            .map(|code| {
                if let Some(key) = Scancode::from_i32(code) {
                    keyboard_state.is_scancode_pressed(key)
                } else {
                    false
                }
            })
            .collect::<Vec<_>>();

        self.current_state = current_state;
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

/// Helper for mouse input
pub struct MouseState {
    mouse_position: Vector2,
    current_button: MouseButton,
    previous_button: MouseButton,
}

impl MouseState {
    pub fn new() -> Self {
        Self {
            mouse_position: Vector2::ZERO,
            current_button: MouseButton::Unknown,
            previous_button: MouseButton::Unknown,
        }
    }

    pub fn get_position(&self) -> &Vector2 {
        &self.mouse_position
    }

    pub fn get_button_value(&self, button: MouseButton) -> bool {
        button == self.current_button
    }

    pub fn get_button_state(&self, button: MouseButton) -> ButtonState {
        let previous = button == self.previous_button;
        let current = button == self.current_button;

        match (previous, current) {
            (false, false) => ButtonState::None,
            (false, true) => ButtonState::Pressed,
            (true, false) => ButtonState::Released,
            (true, true) => ButtonState::Held,
        }
    }
}

/// Wrapper that contains current state of input
pub struct InputState {
    pub keyboard: KeyboardState,
    pub mouse: MouseState,
}

pub struct InputSystem {
    state: InputState,
}

impl InputSystem {
    pub fn initialize() -> Result<Rc<RefCell<Self>>> {
        let keyboard = KeyboardState::new();

        let mouse = MouseState::new();

        let state = InputState { keyboard, mouse };

        let this = Self { state };

        Ok(Rc::new(RefCell::new(this)))
    }

    // Called right before SDL_PollEvents loop
    pub fn prepare_for_update(&mut self) {
        self.state.keyboard.copy_current_to_previous();

        self.state.mouse.previous_button = self.state.mouse.current_button;
    }

    // Called after SDL_PollEvents loop
    pub fn update(&mut self, event_pump: &EventPump) {
        self.state.keyboard.update(&event_pump.keyboard_state());

        let mouse_state = event_pump.mouse_state();
        self.state.mouse.current_button = mouse_state
            .mouse_buttons()
            .map(|(button, _pressed)| button)
            .last()
            .unwrap_or(MouseButton::Unknown);
        self.state.mouse.mouse_position.x = mouse_state.x() as f32;
        self.state.mouse.mouse_position.y = mouse_state.y() as f32;
    }

    pub fn get_state(&self) -> &InputState {
        &self.state
    }
}
