use std::{cell::RefCell, collections::HashMap, rc::Rc};

use anyhow::Result;
use sdl2::{
    controller::GameController, event::Event, keyboard::Scancode, mouse::MouseButton, EventPump,
};

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
    current_button: Vec<MouseButton>,
    previous_button: Vec<MouseButton>,
    is_relative: bool,
    scroll_wheel: Vector2,
}

impl MouseState {
    pub fn new() -> Self {
        Self {
            mouse_position: Vector2::ZERO,
            current_button: vec![],
            previous_button: vec![],
            is_relative: false,
            scroll_wheel: Vector2::ZERO,
        }
    }

    /// Copy current state to previous
    pub fn clone_current_to_previous(&mut self) {
        self.previous_button = self.current_button.clone();
    }

    pub fn get_position(&self) -> &Vector2 {
        &self.mouse_position
    }

    pub fn get_scroll_wheel(&self) -> &Vector2 {
        &self.scroll_wheel
    }

    pub fn get_button_state(&self, button: MouseButton) -> ButtonState {
        let previous = self.get_previous_value(button);
        let current = self.get_button_value(button);

        match (previous, current) {
            (false, false) => ButtonState::None,
            (false, true) => ButtonState::Pressed,
            (true, false) => ButtonState::Released,
            (true, true) => ButtonState::Held,
        }
    }

    pub fn get_button_value(&self, button: MouseButton) -> bool {
        self.current_button.iter().find(|&b| *b == button).is_some()
    }

    fn get_previous_value(&self, button: MouseButton) -> bool {
        self.previous_button
            .iter()
            .find(|&b| *b == button)
            .is_some()
    }
}

/// Wrapper that contains current state of input
pub struct InputState {
    pub keyboard: KeyboardState,
    pub mouse: MouseState,
}

pub struct InputSystem {
    state: InputState,
    controller: Option<GameController>,
}

impl InputSystem {
    pub fn initialize(controller: Option<GameController>) -> Result<Rc<RefCell<Self>>> {
        let keyboard = KeyboardState::new();

        let mouse = MouseState::new();

        let state = InputState { keyboard, mouse };

        let this = Self { state, controller };

        Ok(Rc::new(RefCell::new(this)))
    }

    // Called right before SDL_PollEvents loop
    pub fn prepare_for_update(&mut self) {
        self.state.keyboard.copy_current_to_previous();

        self.state.mouse.clone_current_to_previous();
    }

    // Called after SDL_PollEvents loop
    pub fn update(&mut self, event_pump: &EventPump) {
        self.state.keyboard.update(&event_pump.keyboard_state());

        if self.state.mouse.is_relative {
            let mouse_state = event_pump.relative_mouse_state();
            self.state.mouse.current_button = mouse_state.pressed_mouse_buttons().collect();
            self.state.mouse.mouse_position.x = mouse_state.x() as f32;
            self.state.mouse.mouse_position.y = mouse_state.y() as f32;
        } else {
            let mouse_state = event_pump.mouse_state();
            self.state.mouse.current_button = mouse_state.pressed_mouse_buttons().collect();
            self.state.mouse.mouse_position.x = mouse_state.x() as f32;
            self.state.mouse.mouse_position.y = mouse_state.y() as f32;
        }
    }

    pub fn process_event(&mut self, event: &Event) {
        match *event {
            Event::MouseWheel {
                precise_x,
                precise_y,
                ..
            } => {
                self.state.mouse.scroll_wheel.x = precise_x;
                self.state.mouse.scroll_wheel.y = precise_y;
            }
            _ => {}
        }
    }

    pub fn get_state(&self) -> &InputState {
        &self.state
    }

    pub fn set_relative_mouse_mode(&mut self, is_relative: bool) {
        self.state.mouse.is_relative = is_relative;
    }
}
