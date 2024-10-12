use anyhow::{anyhow, Result};
use sdl2::{
    event::Event,
    keyboard::{KeyboardState, Scancode},
    pixels::Color,
    render::Canvas,
    video::Window,
    EventPump, Sdl,
};

pub struct Game {
    context: Sdl,
    canvas: Canvas<Window>,
    event_pump: EventPump,
    is_running: bool,
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

        Ok(Game {
            context,
            canvas,
            event_pump,
            is_running: true,
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

    fn update_game(&mut self) {}

    fn generate_output(&mut self) {
        self.canvas.set_draw_color(Color::RGBA(0, 0, 255, 255));
        self.canvas.clear();
        self.canvas.present();
    }
}
