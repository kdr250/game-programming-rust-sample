use std::{cell::RefCell, collections::HashMap, path::Path, rc::Rc};

use anyhow::{anyhow, Result};
use sdl2::{
    event::Event,
    image::{InitFlag, LoadTexture},
    keyboard::{KeyboardState, Scancode},
    pixels::Color,
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
    EventPump, TimerSubsystem,
};

use crate::{
    actor::{Actor, DefaultActor, State},
    bg_sprite_component::BGSpriteComponent,
    math::Vector2,
    ship::Ship,
    sprite_component::SpriteComponent,
};

pub struct Game {
    this: Option<Rc<RefCell<Game>>>,
    canvas: Canvas<Window>,
    event_pump: EventPump,
    timer: TimerSubsystem,
    texture_creator: TextureCreator<WindowContext>,
    textures: HashMap<String, Rc<Texture>>,
    sprites: Vec<Rc<RefCell<dyn SpriteComponent>>>,
    actors: Vec<Rc<RefCell<dyn Actor>>>,
    pending_actors: Vec<Rc<RefCell<dyn Actor>>>,
    is_running: bool,
    tick_count: u64,
    updating_actors: bool,
    ship: Option<Rc<RefCell<Ship>>>,
}

impl Game {
    /// Initialize game
    pub fn initialize() -> Result<Rc<RefCell<Game>>> {
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

        let game = Game {
            this: None,
            canvas,
            event_pump,
            timer,
            texture_creator,
            textures: HashMap::new(),
            sprites: vec![],
            actors: vec![],
            pending_actors: vec![],
            is_running: true,
            tick_count: 0,
            updating_actors: false,
            ship: None,
        };

        let result = Rc::new(RefCell::new(game));
        let cloned = result.clone();
        result.borrow_mut().set_this(cloned);
        Self::load_data(result.clone());

        Ok(result)
    }

    /// Runs the game loop until the game is over
    pub fn run_loop(&mut self) {
        while self.is_running {
            self.process_input();
            self.update_game();
            self.generate_output();
        }
    }

    fn load_data(this: Rc<RefCell<Game>>) {
        let ship = Ship::new(this.clone());
        {
            let mut s = ship.borrow_mut();
            s.set_position(Vector2::new(100.0, 384.0));
            s.set_scale(1.5);
        }
        this.borrow_mut().ship = Some(ship);

        let temp = DefaultActor::new(this.clone());
        temp.borrow_mut().set_position(Vector2::new(512.0, 384.0));

        let mut back_ground = BGSpriteComponent::new(temp.clone(), 10);
        {
            let mut bg = back_ground.borrow_mut();
            bg.set_screen_size(Vector2::new(1024.0, 768.0));
            let mut game = this.borrow_mut();
            let bgtexs = vec![
                game.get_texture("Assets/Farback01.png"),
                game.get_texture("Assets/Farback02.png"),
            ];
            bg.set_bg_textures(bgtexs);
            bg.set_scroll_speed(-100.0);
        }

        back_ground = BGSpriteComponent::new(temp.clone(), 50);
        {
            let mut bg = back_ground.borrow_mut();
            bg.set_screen_size(Vector2::new(1024.0, 768.0));
            let mut game = this.borrow_mut();
            let bgtexs = vec![
                game.get_texture("Assets/Stars.png"),
                game.get_texture("Assets/Stars.png"),
            ];
            bg.set_bg_textures(bgtexs);
            bg.set_scroll_speed(-200.0);
        }
    }

    pub fn add_actor(&mut self, actor: Rc<RefCell<dyn Actor>>) {
        if self.updating_actors {
            self.pending_actors.push(actor);
        } else {
            self.actors.push(actor);
        }
    }

    pub fn get_texture(&mut self, file_name: &str) -> Rc<Texture> {
        if let Some(texture) = self.textures.get(&file_name.to_string()) {
            return texture.clone();
        }
        let path = Path::new(env!("OUT_DIR")).join("resources").join(file_name);
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

        self.get_ship().borrow_mut().process_keyboard(state);
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

        // Draw all sprite component
        for sprite in &self.sprites {
            sprite.borrow().draw(&mut self.canvas);
        }

        self.canvas.present();
    }

    pub fn add_sprite(&mut self, sprite: Rc<RefCell<dyn SpriteComponent>>) {
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

    fn set_this(&mut self, this: Rc<RefCell<Game>>) {
        self.this = Some(this);
    }

    fn get_ship(&self) -> Rc<RefCell<Ship>> {
        self.ship.clone().unwrap()
    }
}
