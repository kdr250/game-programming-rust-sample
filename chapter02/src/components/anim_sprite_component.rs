use std::{cell::RefCell, rc::Rc};

use sdl2::render::Texture;

use crate::{
    actors::actor::Actor,
    components::component::{self, Component, State},
    components::sprite_component::{self, SpriteComponent},
};

pub struct AnimSpriteComponent {
    id: u32,
    owner: Rc<RefCell<dyn Actor>>,
    update_order: i32,
    state: State,
    texture: Option<Rc<Texture>>,
    draw_order: i32,
    texture_width: u32,
    texture_height: u32,
    anim_textures: Vec<Rc<Texture>>,
    current_frame: f32,
    anim_fps: f32,
}

impl AnimSpriteComponent {
    pub fn new(owner: Rc<RefCell<dyn Actor>>, draw_order: i32) -> Rc<RefCell<Self>> {
        let this = Self {
            id: component::generate_id(),
            owner: owner.clone(),
            update_order: 100,
            state: State::Active,
            texture: None,
            draw_order,
            texture_height: 0,
            texture_width: 0,
            anim_textures: vec![],
            current_frame: 0.0,
            anim_fps: 24.0,
        };

        let result = Rc::new(RefCell::new(this));

        owner.borrow_mut().add_component(result.clone());

        owner
            .borrow()
            .get_game()
            .borrow_mut()
            .add_sprite(result.clone());

        result
    }

    pub fn set_anim_textures(&mut self, textures: Vec<Rc<Texture>>) {
        self.anim_textures = textures;
        if !self.anim_textures.is_empty() {
            self.current_frame = 0.0;
            self.set_texture(self.anim_textures[0].clone());
        }
    }
}

impl SpriteComponent for AnimSpriteComponent {
    sprite_component::impl_getters_setters! {}
}

impl Component for AnimSpriteComponent {
    fn update(&mut self, delta_time: f32) {
        if self.anim_textures.is_empty() {
            return;
        }

        self.current_frame += self.anim_fps * delta_time;

        while self.current_frame >= self.anim_textures.len() as f32 {
            self.current_frame -= self.anim_textures.len() as f32;
        }

        let texture = self.anim_textures[self.current_frame as usize].clone();

        self.set_texture(texture);
    }

    component::impl_getters_setters! {}
}
