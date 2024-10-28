use std::{cell::RefCell, collections::HashMap, path::Path, rc::Rc};

use anyhow::{Ok, Result};
use sdl2::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

use crate::{
    components::{component::State, sprite_component::SpriteComponent},
    graphics::{shader::Shader, vertex_array::VertexArray},
};

pub struct TextureManager {
    texture_creator: TextureCreator<WindowContext>,
    textures: HashMap<String, Rc<Texture>>,
    sprites: Vec<Rc<RefCell<dyn SpriteComponent>>>,
    sprite_verts: VertexArray,
    shader: Option<Shader>,
}

impl TextureManager {
    pub fn new(texture_creator: TextureCreator<WindowContext>) -> Rc<RefCell<Self>> {
        let this = Self {
            texture_creator,
            textures: HashMap::new(),
            sprites: vec![],
            sprite_verts: Self::create_sprite_verts(),
            shader: None,
        };

        Rc::new(RefCell::new(this))
    }

    fn create_sprite_verts() -> VertexArray {
        let vertices: [f32; 12] = [
            -0.5, 0.5, 0.0, // top left
            0.5, 0.5, 0.0, // top right
            0.5, -0.5, 0.0, // bottom right
            -0.5, -0.5, 0.0, // bottom left
        ];

        let index_buffer: [u32; 6] = [
            0, 1, 2, // right half of triangle
            2, 3, 0, // left half of triangle
        ];

        VertexArray::new(&vertices, 4, &index_buffer, 6)
    }

    pub fn load_shaders(&mut self) -> Result<()> {
        let mut sprite_shader = Shader::new();
        sprite_shader.load("Basic.vert", "Basic.frag")?;
        sprite_shader.set_active();
        self.shader = Some(sprite_shader);
        Ok(())
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
