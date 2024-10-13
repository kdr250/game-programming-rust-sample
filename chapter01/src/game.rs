use anyhow::{anyhow, Result};
use sdl2::{
    event::Event,
    keyboard::{KeyboardState, Scancode},
    pixels::Color,
    rect::Rect,
    render::Canvas,
    video::Window,
    EventPump, TimerSubsystem,
};

const THICKNESS: u32 = 15;
const PADDLE_HEIGHT: f32 = 100.0;

struct Vector2 {
    x: f32,
    y: f32,
}

pub struct Game {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    timer: TimerSubsystem,
    is_running: bool,
    paddle_position: Vector2,
    ball_position: Vector2,
    ball_velocity: Vector2,
    tick_count: u64,
    paddle_dir: i32,
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

        let paddle_position = Vector2 {
            x: 10.0,
            y: 768.0 / 2.0,
        };

        let ball_position = Vector2 {
            x: 1024.0 / 2.0,
            y: 768.0 / 2.0,
        };

        let ball_velocity = Vector2 {
            x: -200.0,
            y: 235.0,
        };

        Ok(Game {
            canvas,
            event_pump,
            timer,
            is_running: true,
            paddle_position,
            ball_position,
            ball_velocity,
            tick_count: 0,
            paddle_dir: 0,
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

    /// Shutdown the game
    pub fn shutdown(&mut self) {}

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

        if self.paddle_dir != 0 {
            self.paddle_position.y += self.paddle_dir as f32 * 300.0 * delta_time;
            self.paddle_position.y = self.paddle_position.y.clamp(
                PADDLE_HEIGHT / 2.0 + THICKNESS as f32,
                768.0 - PADDLE_HEIGHT / 2.0 - THICKNESS as f32,
            );
        }

        self.ball_position.x += self.ball_velocity.x * delta_time;
        self.ball_position.y += self.ball_velocity.y * delta_time;

        let diff = (self.paddle_position.y - self.ball_position.y).abs();

        if diff <= PADDLE_HEIGHT / 2.0
            && self.ball_position.x <= 25.0
            && self.ball_position.x >= 20.0
            && self.ball_velocity.x < 0.0
        {
            self.ball_velocity.x *= -1.0;
        } else if self.ball_position.x <= 0.0 {
            self.is_running = false;
        } else if self.ball_position.x >= 1024.0 - THICKNESS as f32 && self.ball_velocity.x > 0.0 {
            self.ball_velocity.x *= -1.0;
        } else if self.ball_position.y <= THICKNESS as f32 && self.ball_velocity.y < 0.0 {
            self.ball_velocity.y *= -1.0;
        } else if self.ball_position.y >= 768.0 - THICKNESS as f32 && self.ball_velocity.y > 0.0 {
            self.ball_velocity.y *= -1.0;
        }
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
}
