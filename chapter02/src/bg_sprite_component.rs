use std::{cell::RefCell, rc::Rc};

use sdl2::{
    rect::Rect,
    render::{Canvas, Texture},
    video::Window,
};

use crate::{
    actor::Actor,
    component::{self, generate_id, Component, State},
    math::Vector2,
    sprite_component::{self, SpriteComponent},
};

struct BGTexture {
    texture: Rc<Texture>,
    offset: Vector2,
}

pub struct BGSpriteComponent {
    id: u32,
    owner: Rc<RefCell<dyn Actor>>,
    update_order: i32,
    state: State,
    texture: Option<Rc<Texture>>,
    draw_order: i32,
    texture_width: u32,
    texture_height: u32,
    bg_textures: Vec<BGTexture>,
    screen_size: Vector2,
    scroll_speed: f32,
}

impl BGSpriteComponent {
    fn new(owner: Rc<RefCell<dyn Actor>>, draw_order: i32, update_order: i32) -> Rc<RefCell<Self>> {
        let this = Self {
            id: generate_id(),
            owner: owner.clone(),
            update_order,
            state: State::Active,
            texture: None,
            draw_order,
            texture_height: 0,
            texture_width: 0,
            bg_textures: vec![],
            screen_size: Vector2::ZERO,
            scroll_speed: 0.0,
        };

        let result = Rc::new(RefCell::new(this));

        let mut borrowed_onwer = owner.borrow_mut();
        borrowed_onwer.add_component(result.clone());

        let mut borrowed_game = borrowed_onwer.get_game().borrow_mut();
        borrowed_game.add_sprite(result.clone());

        result
    }

    fn set_bg_textures(&mut self, textures: Vec<Rc<Texture>>) {
        let mut count = 0;
        for texture in textures {
            let temp = BGTexture {
                texture,
                offset: Vector2::new(count as f32 * self.screen_size.x, 0.0),
            };
            self.bg_textures.push(temp);
            count += 1;
        }
    }

    fn set_screen_size(&mut self, size: Vector2) {
        self.screen_size = size;
    }

    fn set_scroll_speed(&mut self, speed: f32) {
        self.scroll_speed = speed;
    }

    fn get_scroll_speed(&self) -> f32 {
        self.scroll_speed
    }
}

impl SpriteComponent for BGSpriteComponent {
    sprite_component::impl_getters_setters! {}

    fn draw(&mut self, canvas: &mut Canvas<Window>) {
        let owner = self.get_owner().borrow();
        let width = self.screen_size.x;
        let height = self.screen_size.y;
        for bg in &self.bg_textures {
            let rect = Rect::new(
                (owner.get_position().x - width / 2.0 + bg.offset.x) as i32,
                (owner.get_position().y - height / 2.0 + bg.offset.y) as i32,
                width as u32,
                height as u32,
            );
            canvas.copy(&bg.texture, None, rect).unwrap();
        }
    }
}

impl Component for BGSpriteComponent {
    fn update(&mut self, delta_time: f32) {
        let len = self.bg_textures.len();
        for bg in &mut self.bg_textures {
            bg.offset.x += self.scroll_speed * delta_time;
            if bg.offset.x < -self.screen_size.x {
                bg.offset.x = (len - 1) as f32 * self.screen_size.x - 1.0;
            }
        }
    }

    component::impl_getters_setters! {}
}
