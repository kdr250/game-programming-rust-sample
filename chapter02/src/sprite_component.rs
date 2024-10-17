use std::{cell::RefCell, rc::Rc};

use sdl2::{
    rect::Rect,
    render::{Canvas, Texture},
    video::Window,
};

use crate::{
    actor::Actor,
    component::{self, generate_id, Component, State},
    math::{self},
};

pub struct SpriteComponent {
    id: u32,
    owner: Rc<RefCell<dyn Actor>>,
    update_order: i32,
    state: State,
    texture: Option<Texture>,
    draw_order: i32,
    texture_width: u32,
    texture_height: u32,
}

impl SpriteComponent {
    pub fn new(
        owner: Rc<RefCell<dyn Actor>>,
        draw_order: i32,
        update_order: i32,
    ) -> Rc<RefCell<Self>> {
        let this = Self {
            id: generate_id(),
            owner: owner.clone(),
            update_order,
            state: State::Active,
            texture: None,
            draw_order,
            texture_height: 0,
            texture_width: 0,
        };

        let result = Rc::new(RefCell::new(this));

        let mut borrowed_onwer = owner.borrow_mut();
        borrowed_onwer.add_component(result.clone());

        let mut borrowed_game = borrowed_onwer.get_game().borrow_mut();
        borrowed_game.add_sprite(result.clone());

        result
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        if let Some(texture) = &self.texture {
            let owner = self.owner.borrow();
            let width = self.texture_width as f32 * owner.get_scale();
            let height = self.texture_height as f32 * owner.get_scale();
            let rect = Rect::new(
                (owner.get_position().x - width / 2.0) as i32,
                (owner.get_position().y - height / 2.0) as i32,
                width as u32,
                height as u32,
            );

            canvas
                .copy_ex(
                    texture,
                    None,
                    Some(rect),
                    -math::to_degrees(owner.get_rotation()) as f64,
                    None,
                    false,
                    false,
                )
                .unwrap();
        }
    }

    pub fn get_texture(&self) -> &Texture {
        self.texture.as_ref().unwrap()
    }

    pub fn set_texture(&mut self, texture: Texture) {
        let query = texture.query();
        self.texture_height = query.height;
        self.texture_width = query.width;
        self.texture = Some(texture);
    }

    pub fn get_draw_order(&self) -> i32 {
        self.draw_order
    }

    pub fn get_texture_height(&self) -> u32 {
        self.texture_height
    }

    pub fn get_texture_width(&self) -> u32 {
        self.texture_width
    }
}

impl Component for SpriteComponent {
    fn update(&mut self, delta_time: f32) {}

    component::impl_getters_setters! {}
}
