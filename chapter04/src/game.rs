use std::{cell::RefCell, rc::Rc};

use anyhow::{anyhow, Result};
use sdl2::{
    event::Event,
    image::InitFlag,
    keyboard::{KeyboardState, Scancode},
    mouse::MouseButton,
    pixels::Color,
    render::Canvas,
    video::Window,
    EventPump, TimerSubsystem,
};

use crate::system::{entity_manager::EntityManager, texture_manager::TextureManager};

pub struct Game {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    timer: TimerSubsystem,
    texture_manager: Rc<RefCell<TextureManager>>,
    entity_manager: Rc<RefCell<EntityManager>>,
    is_running: bool,
    tick_count: u64,
}

impl Game {
    /// Initialize game
    pub fn initialize() -> Result<Game> {
        let sdl = sdl2::init().map_err(|e| anyhow!(e))?;

        let video_system = sdl.video().map_err(|e| anyhow!(e))?;

        let window = video_system
            .window("Game Programming in Rust", 1024, 768)
            .position(100, 100)
            .build()?;

        let canvas = window.into_canvas().build()?;

        let event_pump = sdl.event_pump().map_err(|e| anyhow!(e))?;

        let timer = sdl.timer().map_err(|e| anyhow!(e))?;

        let _image_context = sdl2::image::init(InitFlag::PNG).map_err(|e| anyhow!(e))?;
        let texture_creator = canvas.texture_creator();
        let texture_manager = TextureManager::new(texture_creator);

        let entity_manager = EntityManager::new();
        EntityManager::load_data(entity_manager.clone(), texture_manager.clone());

        let game = Game {
            canvas,
            event_pump,
            timer,
            texture_manager,
            entity_manager,
            is_running: true,
            tick_count: 0,
        };

        Ok(game)
    }

    /// Runs the game loop until the game is over
    pub fn run_loop(&mut self) {
        while self.is_running {
            self.process_input();
            self.update_game();
            self.generate_output();
        }
    }

    /// Herlper functions for the game loop
    fn process_input(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    self.is_running = false;
                    break;
                }
                _ => {}
            }
        }

        let state = KeyboardState::new(&self.event_pump);
        if state.is_scancode_pressed(Scancode::Escape) {
            self.is_running = false;
        }

        if state.is_scancode_pressed(Scancode::B) {
            let grid = self.entity_manager.borrow().get_grid();
            grid.borrow_mut().build_tower();
        }

        // process mouse
        let button = self.event_pump.mouse_state();
        if button.is_mouse_button_pressed(MouseButton::Left) {
            let grid = self.entity_manager.borrow().get_grid();
            grid.borrow_mut().process_click(button.x(), button.y());
        }

        self.entity_manager.borrow_mut().set_updating_actors(true);
        let actors = self.entity_manager.borrow().get_actors().clone();
        for actor in actors {
            actor.borrow_mut().process_input(&state);
        }
        self.entity_manager.borrow_mut().set_updating_actors(false);
    }

    fn update_game(&mut self) {
        while self.timer.ticks64() < self.tick_count + 16 {}

        let mut delta_time = (self.timer.ticks64() - self.tick_count) as f32 / 1000.0;

        delta_time = delta_time.min(0.05);

        self.tick_count = self.timer.ticks64();

        self.entity_manager.borrow_mut().set_updating_actors(true);
        let actors = self.entity_manager.borrow().get_actors().clone();
        for actor in actors {
            actor.borrow_mut().update(delta_time);
        }
        self.entity_manager.borrow_mut().set_updating_actors(false);

        self.entity_manager.borrow_mut().flush_actors();
        self.texture_manager.borrow_mut().flush_sprites();
    }

    fn generate_output(&mut self) {
        self.canvas.set_draw_color(Color::RGBA(34, 139, 34, 255));
        self.canvas.clear();

        // Draw all sprite component
        for sprite in self.texture_manager.borrow().get_sprites() {
            sprite.borrow().draw(&mut self.canvas);
        }

        self.canvas.present();
    }
}
