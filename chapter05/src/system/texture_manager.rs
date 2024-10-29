use std::{cell::RefCell, collections::HashMap, path::Path, rc::Rc};

use anyhow::{Ok, Result};

use crate::{
    components::{component::State, sprite_component::SpriteComponent},
    graphics::{shader::Shader, texture::Texture, vertex_array::VertexArray},
    math::matrix4::Matrix4,
};

pub struct TextureManager {
    textures: HashMap<String, Rc<Texture>>,
    sprites: Vec<Rc<RefCell<dyn SpriteComponent>>>,
    pub sprite_verts: VertexArray,
    pub sprite_shader: Shader,
}

impl TextureManager {
    pub fn new() -> Rc<RefCell<Self>> {
        let this = Self {
            textures: HashMap::new(),
            sprites: vec![],
            sprite_verts: Self::create_sprite_verts(),
            sprite_shader: Shader::new(),
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
        self.sprite_shader.load("Transform.vert", "Basic.frag")?;
        self.sprite_shader.set_active();

        let view_proj = Matrix4::create_simple_view_proj(1024.0, 768.0);
        self.sprite_shader
            .set_matrix_uniform("uViewProj", view_proj);

        Ok(())
    }

    pub fn get_texture(&mut self, file_name: &str) -> Rc<Texture> {
        if let Some(texture) = self.textures.get(&file_name.to_string()) {
            return texture.clone();
        }

        let mut texture = Texture::new();
        if texture.load(file_name).is_ok() {
            let result = Rc::new(texture);
            self.textures.insert(file_name.to_string(), result.clone());
            return result;
        }

        panic!("failed to get texture: {}", file_name);
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
