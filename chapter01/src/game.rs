use anyhow::{anyhow, Result};
use sdl2::{
    event::Event,
    keyboard::{KeyboardState, Scancode},
    pixels::Color,
    rect::Rect,
    render::Canvas,
    video::Window,
    EventPump, Sdl, TimerSubsystem,
};

const THICKNESS: u32 = 15;

struct Vector2 {
    x: f32,
    y: f32,
}

pub struct Game {
    context: Sdl,
    canvas: Canvas<Window>,
    event_pump: EventPump,
    timer: TimerSubsystem,
    is_running: bool,
    paddle_position: Vector2,
    ball_position: Vector2,
    tick_count: u64,
}

impl Game {
    /// Initialize game
    pub fn initialize() -> Result<Game> {
        let context = sdl2::init().map_err(|e| anyhow!(e))?;

        let video_system = context.video().map_err(|e| anyhow!(e))?;

        let window = video_system
            .window("Game Programming in Rust", 1024, 768)
            .position(100, 100)
            .build()?;

        let canvas = window.into_canvas().build()?;

        let event_pump = context.event_pump().map_err(|e| anyhow!(e))?;

        let timer = context.timer().map_err(|e| anyhow!(e))?;

        let paddle_position = Vector2 {
            x: 10.0,
            y: 768.0 / 2.0,
        };

        let ball_position = Vector2 {
            x: 1024.0 / 2.0,
            y: 768.0 / 2.0,
        };

        Ok(Game {
            context,
            canvas,
            event_pump,
            timer,
            is_running: true,
            paddle_position,
            ball_position,
            tick_count: 0,
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
    }

    fn update_game(&mut self) {
        while self.timer.ticks64() < self.tick_count + 16 {}

        let mut delta_time = (self.timer.ticks64() - self.tick_count) as f32 / 1000.0;

        delta_time = delta_time.min(0.05);

        self.tick_count = self.timer.ticks64();

        // TODO: Update game objects
    }

    fn generate_output(&mut self) {
        self.canvas.set_draw_color(Color::RGBA(0, 0, 255, 255));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));

        let wall = Rect::new(0, 0, 1024, THICKNESS);
        self.canvas.fill_rect(wall).unwrap();

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
