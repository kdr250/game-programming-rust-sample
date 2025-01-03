extern crate gl;

use std::{cell::RefCell, rc::Rc};

use anyhow::{anyhow, Result};
use sdl2::{
    event::Event,
    keyboard::{KeyboardState, Scancode},
    EventPump, TimerSubsystem,
};

use crate::system::{
    asset_manager::AssetManager, entity_manager::EntityManager, renderer::Renderer,
};

pub struct Game {
    renderer: Rc<RefCell<Renderer>>,
    event_pump: EventPump,
    timer: TimerSubsystem,
    asset_manager: Rc<RefCell<AssetManager>>,
    entity_manager: Rc<RefCell<EntityManager>>,
    is_running: bool,
    tick_count: u64,
}

impl Game {
    /// Initialize game
    pub fn initialize() -> Result<Game> {
        let sdl = sdl2::init().map_err(|e| anyhow!(e))?;
        let video_system = sdl.video().map_err(|e| anyhow!(e))?;

        let renderer = Renderer::initialize(video_system, (1024.0, 768.0))?;

        let event_pump = sdl.event_pump().map_err(|e| anyhow!(e))?;

        let timer = sdl.timer().map_err(|e| anyhow!(e))?;

        let asset_manager = renderer.borrow().get_asset_manager().clone();
        let entity_manager = EntityManager::new();
        EntityManager::load_data(
            entity_manager.clone(),
            asset_manager.clone(),
            renderer.clone(),
        );

        let game = Game {
            renderer,
            event_pump,
            timer,
            asset_manager,
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

        self.entity_manager.borrow_mut().set_updating_actors(true);
        let actors = self.entity_manager.borrow().get_actors().clone();
        for actor in actors {
            actor.borrow_mut().process_input(&state);
        }
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

        let pending_actors = self.entity_manager.borrow().get_pending_actors().clone();
        for pending in pending_actors {
            pending.borrow_mut().compute_world_transform();
            self.entity_manager.borrow_mut().add_actor(pending.clone());
        }

        self.entity_manager.borrow_mut().flush_actors();
        self.asset_manager.borrow_mut().flush_sprites();
    }

    fn generate_output(&mut self) {
        self.renderer.borrow_mut().draw();
    }
}
