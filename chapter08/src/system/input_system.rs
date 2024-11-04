use std::{cell::RefCell, collections::HashMap, rc::Rc};

use anyhow::Result;
use sdl2::{
    controller::{Axis, Button, GameController},
    event::Event,
    keyboard::Scancode,
    mouse::MouseButton,
    sys::SDL_GameControllerButton,
    EventPump,
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
    current_buttons: Vec<MouseButton>,
    previous_buttons: Vec<MouseButton>,
    is_relative: bool,
    scroll_wheel: Vector2,
}

impl MouseState {
    pub fn new() -> Self {
        Self {
            mouse_position: Vector2::ZERO,
            current_buttons: vec![],
            previous_buttons: vec![],
            is_relative: false,
            scroll_wheel: Vector2::ZERO,
        }
    }

    pub fn update(&mut self, event_pump: &EventPump) {
        if self.is_relative {
            let mouse_state = event_pump.relative_mouse_state();
            self.current_buttons = mouse_state.pressed_mouse_buttons().collect();
            self.mouse_position.x = mouse_state.x() as f32;
            self.mouse_position.y = mouse_state.y() as f32;
        } else {
            let mouse_state = event_pump.mouse_state();
            self.current_buttons = mouse_state.pressed_mouse_buttons().collect();
            self.mouse_position.x = mouse_state.x() as f32;
            self.mouse_position.y = mouse_state.y() as f32;
        }
    }

    /// Copy current state to previous
    pub fn clone_current_to_previous(&mut self) {
        self.previous_buttons = self.current_buttons.clone();
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
        self.current_buttons
            .iter()
            .find(|&b| *b == button)
            .is_some()
    }

    fn get_previous_value(&self, button: MouseButton) -> bool {
        self.previous_buttons
            .iter()
            .find(|&b| *b == button)
            .is_some()
    }
}

/// Helper for controller input
pub struct ControllerState {
    current_buttons: [bool; SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_MAX as usize],
    previous_buttons: [bool; SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_MAX as usize],
    left_stick: Vector2,
    right_stick: Vector2,
    left_trigger: f32,
    right_trigger: f32,
    is_connected: bool,
}

impl ControllerState {
    pub fn new(controller: &Option<GameController>) -> Self {
        Self {
            current_buttons: [false; SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_MAX as usize],
            previous_buttons: [false; SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_MAX as usize],
            left_stick: Vector2::ZERO,
            right_stick: Vector2::ZERO,
            left_trigger: 0.0,
            right_trigger: 0.0,
            is_connected: controller.is_some(),
        }
    }

    pub fn update(&mut self, game_controller: &GameController) {
        // Buttons
        for i in 0..SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_MAX as usize {
            let button = unsafe { std::mem::transmute::<_, Button>(i as i32) };
            self.current_buttons[i] = game_controller.button(Button::from(button));
        }

        // Triggers
        self.left_trigger = InputSystem::filter_1d(game_controller.axis(Axis::TriggerLeft) as i32);
        self.right_trigger =
            InputSystem::filter_1d(game_controller.axis(Axis::TriggerRight) as i32);

        // Sticks
        let x = game_controller.axis(Axis::LeftX) as i32;
        let y = game_controller.axis(Axis::LeftY) as i32;
        self.left_stick = InputSystem::filter_2d(x, y);

        let x = game_controller.axis(Axis::RightX) as i32;
        let y = game_controller.axis(Axis::RightY) as i32;
        self.right_stick = InputSystem::filter_2d(x, y);
    }

    pub fn get_button_state(&self, button: Button) -> ButtonState {
        let previous = self.get_previous_value(button);
        let current = self.get_button_value(button);

        match (previous, current) {
            (false, false) => ButtonState::None,
            (false, true) => ButtonState::Pressed,
            (true, false) => ButtonState::Released,
            (true, true) => ButtonState::Held,
        }
    }

    pub fn get_button_value(&self, button: Button) -> bool {
        self.current_buttons[button as usize]
    }

    fn get_previous_value(&self, button: Button) -> bool {
        self.previous_buttons[button as usize]
    }

    pub fn get_is_connected(&self) -> bool {
        self.is_connected
    }

    pub fn get_left_trigger(&self) -> f32 {
        self.left_trigger
    }

    pub fn get_right_trigger(&self) -> f32 {
        self.right_trigger
    }

    pub fn copy_current_to_previous(&mut self) {
        self.previous_buttons.copy_from_slice(&self.current_buttons);
    }
}

/// Wrapper that contains current state of input
pub struct InputState {
    pub keyboard: KeyboardState,
    pub mouse: MouseState,
    pub controller: ControllerState,
}

pub struct InputSystem {
    state: InputState,
    game_controller: Option<GameController>,
}

impl InputSystem {
    pub fn initialize(game_controller: Option<GameController>) -> Result<Rc<RefCell<Self>>> {
        let keyboard = KeyboardState::new();

        let mouse = MouseState::new();

        let controller = ControllerState::new(&game_controller);

        let state = InputState {
            keyboard,
            mouse,
            controller,
        };

        let this = Self {
            state,
            game_controller,
        };

        Ok(Rc::new(RefCell::new(this)))
    }

    // Called right before SDL_PollEvents loop
    pub fn prepare_for_update(&mut self) {
        self.state.keyboard.copy_current_to_previous();

        self.state.mouse.clone_current_to_previous();

        self.state.controller.copy_current_to_previous();
    }

    // Called after SDL_PollEvents loop
    pub fn update(&mut self, event_pump: &EventPump) {
        self.state.keyboard.update(&event_pump.keyboard_state());

        self.state.mouse.update(&event_pump);

        if let Some(game_controller) = &self.game_controller {
            self.state.controller.update(game_controller);
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

    pub fn filter_1d(input: i32) -> f32 {
        // A value < deadZone is interpreted as 0%. A value > maxValue is interpreted as 100%
        let dead_zone = 250;
        let max_value = 30000;

        let mut result = 0.0;

        let abs_value = input.abs();
        if abs_value > dead_zone {
            // compute fractional value between deadZone and maxValue
            result = (abs_value - dead_zone) as f32 / (max_value - dead_zone) as f32;
            result = if input > 0 { result } else { -result };
            result = result.clamp(-1.0, 1.0);
        }

        result
    }

    pub fn filter_2d(input_x: i32, input_y: i32) -> Vector2 {
        let dead_zone = 8000.0;
        let max_value = 30000.0;

        let dir = Vector2::new(input_x as f32, input_y as f32);

        let length = dir.length();

        let result = if length < dead_zone {
            Vector2::ZERO
        } else {
            let mut f = (length - dead_zone) / (max_value - dead_zone);
            f = f.clamp(0.0, 1.0);
            dir * (f / length)
        };

        result
    }
}
