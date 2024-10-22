use std::{cell::RefCell, collections::HashMap, path::Path, rc::Rc};

use sdl2::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

use crate::components::{component::State, sprite_component::SpriteComponent};

pub struct TextureManager {
    texture_creator: TextureCreator<WindowContext>,
    textures: HashMap<String, Rc<Texture>>,
    sprites: Vec<Rc<RefCell<dyn SpriteComponent>>>,
}

impl TextureManager {
    pub fn new(texture_creator: TextureCreator<WindowContext>) -> Rc<RefCell<Self>> {
        let this = Self {
            texture_creator,
            textures: HashMap::new(),
            sprites: vec![],
        };

        Rc::new(RefCell::new(this))
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

    pub fn get_sprites(&self) -> &Vec<Rc<RefCell<dyn SpriteComponent>>> {
        &self.sprites
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

    pub fn flush_sprites(&mut self) {
        self.sprites
            .retain(|sprite| *sprite.borrow().get_state() == State::Active {});
    }
}
