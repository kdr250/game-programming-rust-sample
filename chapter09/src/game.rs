extern crate gl;

use std::{cell::RefCell, rc::Rc};

use anyhow::{anyhow, Result};
use sdl2::{
    event::Event,
    keyboard::{KeyboardState, Scancode},
    EventPump, TimerSubsystem,
};

use crate::{
    actors::{
        actor::{self, Actor, DefaultActor},
        follow_actor::FollowActor,
        fps_actor::FPSActor,
        orbit_actor::{self, OrbitActor},
        spline_actor::{self, SplineActor},
    },
    math::vector3::Vector3,
    system::{
        asset_manager::AssetManager, audio_system::AudioSystem, entity_manager::EntityManager,
        renderer::Renderer, sound_event::SoundEvent,
    },
};

pub struct Game {
    renderer: Rc<RefCell<Renderer>>,
    event_pump: EventPump,
    timer: TimerSubsystem,
    asset_manager: Rc<RefCell<AssetManager>>,
    entity_manager: Rc<RefCell<EntityManager>>,
    audio_system: Rc<RefCell<AudioSystem>>,
    is_running: bool,
    tick_count: u64,
    music_event: SoundEvent,
    reverb_snap: Option<SoundEvent>,
    fps_actor: Rc<RefCell<FPSActor>>,
    follow_actor: Rc<RefCell<FollowActor>>,
    orbit_actor: Rc<RefCell<OrbitActor>>,
    spline_actor: Rc<RefCell<SplineActor>>,
    start_sphere: Rc<RefCell<DefaultActor>>,
    end_sphere: Rc<RefCell<DefaultActor>>,
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

        let (fps_actor, follow_actor, orbit_actor, spline_actor, start_sphere, end_sphere) =
            EntityManager::load_data(
                entity_manager.clone(),
                asset_manager.clone(),
                renderer.clone(),
                audio_system.clone(),
            );

        let mut game = Game {
            renderer,
            event_pump,
            timer,
            asset_manager,
            entity_manager,
            audio_system,
            is_running: true,
            tick_count: 0,
            music_event,
            reverb_snap: None,
            fps_actor,
            follow_actor,
            orbit_actor,
            spline_actor,
            start_sphere,
            end_sphere,
        };

        game.change_camera(1);

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
        let mut scancodes = vec![];
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
                        scancodes.push(scancode.unwrap());
                    }
                }
                _ => {}
            }
        }

        for scancode in scancodes {
            self.handle_key_pressed(scancode);
        }

        let state = KeyboardState::new(&self.event_pump);
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

    fn handle_key_pressed(&mut self, key: Scancode) {
        match key {
            Scancode::Minus => {
                // Reduce master volume
                let mut volume = self.audio_system.borrow().get_bus_volume("bus:/");
                volume = f32::max(0.0, volume - 0.1);
                self.audio_system
                    .borrow_mut()
                    .set_bus_volume("bus:/", volume);
            }
            Scancode::Equals => {
                // Increase master volume
                let mut volume = self.audio_system.borrow().get_bus_volume("bus:/");
                volume = f32::min(1.0, volume + 0.1);
                self.audio_system
                    .borrow_mut()
                    .set_bus_volume("bus:/", volume);
            }
            Scancode::E => {
                self.audio_system
                    .borrow_mut()
                    .play_event("event:/Explosion2D");
            }
            Scancode::M => {
                self.music_event.set_paused(!self.music_event.get_paused());
            }
            Scancode::R => {
                // FIXME: An error will happen when switching four times...
                if let Some(reverb) = &mut self.reverb_snap {
                    if reverb.is_valid() {
                        reverb.stop(true);
                        return;
                    }
                }
                let reverb = self
                    .audio_system
                    .borrow_mut()
                    .play_event("snapshot:/WithReverb");
                self.reverb_snap = Some(reverb);
            }
            Scancode::Num1 | Scancode::Num2 | Scancode::Num3 | Scancode::Num4 => {
                self.change_camera(key as i32 - 29);
            }
            Scancode::P => {
                // Get start point (in center of screen on near plane)
                let mut screen_point = Vector3::new(0.0, 0.0, 0.0);
                let start = self.renderer.borrow().unproject(screen_point.clone());
                // Get end point (in center of screen, between near and far)
                screen_point.z = 0.9;
                let end = self.renderer.borrow().unproject(screen_point);
                // Set spheres to points
                self.start_sphere.borrow_mut().set_position(start);
                self.end_sphere.borrow_mut().set_position(end);
            }
            _ => {}
        };
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

        self.audio_system.borrow_mut().update(delta_time);
    }

    fn generate_output(&mut self) {
        self.renderer.borrow_mut().draw();
    }

    fn change_camera(&mut self, mode: i32) {
        // Disable everything
        self.fps_actor.borrow_mut().set_state(actor::State::Paused);
        self.fps_actor.borrow_mut().set_visible(false);
        self.follow_actor
            .borrow_mut()
            .set_state(actor::State::Paused);
        self.follow_actor.borrow_mut().set_visible(false);
        self.orbit_actor
            .borrow_mut()
            .set_state(actor::State::Paused);
        self.orbit_actor.borrow_mut().set_visible(false);
        self.spline_actor
            .borrow_mut()
            .set_state(actor::State::Paused);

        // Enable the camera specified by the mode
        match mode {
            4 => {
                self.spline_actor
                    .borrow_mut()
                    .set_state(actor::State::Active);
                self.spline_actor.borrow_mut().restart_spline();
            }
            3 => {
                self.orbit_actor
                    .borrow_mut()
                    .set_state(actor::State::Active);
                self.orbit_actor.borrow_mut().set_visible(true);
            }
            2 => {
                self.follow_actor
                    .borrow_mut()
                    .set_state(actor::State::Active);
                self.follow_actor.borrow_mut().set_visible(true);
            }
            1 | _ => {
                self.fps_actor.borrow_mut().set_state(actor::State::Active);
                self.fps_actor.borrow_mut().set_visible(true);
            }
        }
    }
}
