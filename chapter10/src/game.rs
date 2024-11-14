extern crate gl;

use std::{cell::RefCell, rc::Rc};

use anyhow::{anyhow, Result};
use sdl2::{event::Event, keyboard::Scancode, EventPump, TimerSubsystem};

use crate::{
    actors::fps_actor::FPSActor,
    system::{
        asset_manager::AssetManager, audio_system::AudioSystem, entity_manager::EntityManager,
        phys_world::PhysWorld, renderer::Renderer, sound_event::SoundEvent,
    },
};

pub struct Game {
    renderer: Rc<RefCell<Renderer>>,
    event_pump: EventPump,
    timer: TimerSubsystem,
    asset_manager: Rc<RefCell<AssetManager>>,
    entity_manager: Rc<RefCell<EntityManager>>,
    audio_system: Rc<RefCell<AudioSystem>>,
    phys_world: Rc<RefCell<PhysWorld>>,
    is_running: bool,
    tick_count: u64,
    music_event: SoundEvent,
    reverb_snap: Option<SoundEvent>,
    fps_actor: Rc<RefCell<FPSActor>>,
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

        let audio_system = AudioSystem::initialize(asset_manager.clone())?;
        let music_event = audio_system.borrow_mut().play_event("event:/Music");

        let phys_world = PhysWorld::new();

        let camera_actor = EntityManager::load_data(
            entity_manager.clone(),
            asset_manager.clone(),
            renderer.clone(),
            audio_system.clone(),
            phys_world.clone(),
        );

        let game = Game {
            renderer,
            event_pump,
            timer,
            asset_manager,
            entity_manager,
            audio_system,
            phys_world,
            is_running: true,
            tick_count: 0,
            music_event,
            reverb_snap: None,
            fps_actor: camera_actor,
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
                Event::KeyDown {
                    scancode, repeat, ..
                } => {
                    if !repeat && scancode.is_some() {
                        if let Some(reverb) = Game::handle_key_pressed(
                            scancode.unwrap(),
                            self.audio_system.clone(),
                            self.fps_actor.clone(),
                        ) {
                            self.reverb_snap = Some(reverb);
                        }
                    }
                }
                _ => {}
            }
        }

        let state = self.event_pump.keyboard_state();
        if state.is_scancode_pressed(Scancode::Escape) {
            self.is_running = false;
        }

        let mouse_state = self.event_pump.relative_mouse_state();

        self.entity_manager.borrow_mut().set_updating_actors(true);
        let actors = self.entity_manager.borrow().get_actors().clone();
        for actor in actors {
            actor.borrow_mut().process_input(&state, &mouse_state);
        }
    }

    fn handle_key_pressed(
        key: Scancode,
        audio_system: Rc<RefCell<AudioSystem>>,
        fps_actor: Rc<RefCell<FPSActor>>,
    ) -> Option<SoundEvent> {
        match key {
            Scancode::Minus => {
                // Reduce master volume
                let mut volume = audio_system.borrow().get_bus_volume("bus:/");
                volume = f32::max(0.0, volume - 0.1);
                audio_system.borrow_mut().set_bus_volume("bus:/", volume);
            }
            Scancode::Equals => {
                // Increase master volume
                let mut volume = audio_system.borrow().get_bus_volume("bus:/");
                volume = f32::min(1.0, volume + 0.1);
                audio_system.borrow_mut().set_bus_volume("bus:/", volume);
            }
            Scancode::B => {
                fps_actor.borrow_mut().shoot();
            }
            _ => {}
        };
        return None;
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
        self.phys_world.borrow_mut().flush_boxes();

        self.audio_system.borrow_mut().update(delta_time);
    }

    fn generate_output(&mut self) {
        self.renderer.borrow_mut().draw();
    }
}
