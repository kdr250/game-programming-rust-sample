use std::{cell::RefCell, collections::HashMap, path::Path, rc::Rc};

use anyhow::{anyhow, Result};
use sdl2::{
    event::Event,
    image::{InitFlag, LoadTexture},
    keyboard::{KeyboardState, Scancode},
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
    EventPump, TimerSubsystem,
};

use crate::{
    actor::{Actor, State},
    component::Component,
    math::*,
    sprite_component::SpriteComponent,
};

const THICKNESS: u32 = 15;
const PADDLE_HEIGHT: f32 = 100.0;

#[cfg(feature = "unsafe_textures")]
pub struct Game {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    timer: TimerSubsystem,
    texture_creator: TextureCreator<WindowContext>,
    textures: HashMap<String, Rc<Texture>>,
    sprites: Vec<Rc<RefCell<SpriteComponent>>>,
    is_running: bool,
    paddle_position: Vector2,
    ball_position: Vector2,
    ball_velocity: Vector2,
    tick_count: u64,
    paddle_dir: i32,
    updating_actors: bool,
    actors: Vec<Rc<RefCell<dyn Actor>>>,
    pending_actors: Vec<Rc<RefCell<dyn Actor>>>,
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

        let paddle_position = Vector2::new(10.0, 768.0 / 2.0);

        let ball_position = Vector2::new(1024.0 / 2.0, 768.0 / 2.0);

        let ball_velocity = Vector2::new(-200.0, 235.0);

        Ok(Game {
            canvas,
            event_pump,
            timer,
            texture_creator,
            textures: HashMap::new(),
            sprites: vec![],
            is_running: true,
            paddle_position,
            ball_position,
            ball_velocity,
            tick_count: 0,
            paddle_dir: 0,
            updating_actors: false,
            actors: vec![],
            pending_actors: vec![],
        })
    }

    /// Runs the game loop until the game is over
    pub fn run_loop(&mut self) {
        while self.is_running {
            self.process_input();
            self.update_game();
            self.generate_output();
        }
    }

    fn add_actor(&mut self, actor: Rc<RefCell<dyn Actor>>) {
        if self.updating_actors {
            self.pending_actors.push(actor);
        } else {
            self.actors.push(actor);
        }
    }

    fn get_texture(&mut self, file_name: &str) -> Rc<Texture> {
        if let Some(texture) = self.textures.get(&file_name.to_string()) {
            return texture.clone();
        }
        let path = Path::new(file_name);
        let texture = self
            .texture_creator
            .load_texture(path)
            .expect(&format!("Failed to load texture {}", file_name));
        let result = Rc::new(texture);
        self.textures.insert(file_name.to_string(), result.clone());
        result
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

        self.paddle_dir = 0;
        if state.is_scancode_pressed(Scancode::W) {
            self.paddle_dir -= 1;
        }
        if state.is_scancode_pressed(Scancode::S) {
            self.paddle_dir += 1;
        }
    }

    fn update_game(&mut self) {
        while self.timer.ticks64() < self.tick_count + 16 {}

        let mut delta_time = (self.timer.ticks64() - self.tick_count) as f32 / 1000.0;

        delta_time = delta_time.min(0.05);

        self.tick_count = self.timer.ticks64();

        self.updating_actors = true;
        for actor in &self.actors {
            actor.borrow_mut().update(delta_time);
        }
        self.updating_actors = false;

        for pending in &self.pending_actors {
            self.actors.push(pending.clone());
        }
        self.pending_actors.clear();

        self.actors
            .retain(|actor| *actor.borrow().get_state() != State::Dead);
    }

    fn generate_output(&mut self) {
        self.canvas.set_draw_color(Color::RGBA(0, 0, 255, 255));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));

        // Draw top wall
        let mut wall = Rect::new(0, 0, 1024, THICKNESS);
        self.canvas.fill_rect(wall).unwrap();
        // Draw bottom wall
        wall.y = 768 - THICKNESS as i32;
        self.canvas.fill_rect(wall).unwrap();
        // Draw right wall
        wall.x = 1024 - THICKNESS as i32;
        wall.y = 0;
        wall.w = THICKNESS as i32;
        wall.h = 1024;
        self.canvas.fill_rect(wall).unwrap();

        // Draw paddle
        let paddle = Rect::new(
            self.paddle_position.x as i32,
            self.paddle_position.y as i32 - PADDLE_HEIGHT as i32 / 2,
            THICKNESS,
            PADDLE_HEIGHT as u32,
        );
        self.canvas.fill_rect(paddle).unwrap();

        // Draw ball
        let ball = Rect::new(
            self.ball_position.x as i32 - THICKNESS as i32 / 2,
            self.ball_position.y as i32 - THICKNESS as i32 / 2,
            THICKNESS,
            THICKNESS,
        );
        self.canvas.fill_rect(ball).unwrap();

        self.canvas.present();
    }

    pub fn add_sprite(&mut self, sprite: Rc<RefCell<SpriteComponent>>) {
        let draw_order = sprite.borrow().get_draw_order();
        if let Some(index) = self
            .sprites
            .iter()
            .position(|s| s.borrow().get_draw_order() > draw_order)
        {
            self.sprites.insert(index, sprite);
        } else {
            self.sprites.push(sprite);
        }
    }
}
