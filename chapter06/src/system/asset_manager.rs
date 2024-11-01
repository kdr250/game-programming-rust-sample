use std::{cell::RefCell, collections::HashMap, rc::Rc};

use anyhow::{Ok, Result};

use crate::{
    components::{
        component::State, mesh_component::MeshComponent, sprite_component::SpriteComponent,
    },
    graphics::{mesh::Mesh, shader::Shader, texture::Texture, vertex_array::VertexArray},
    math::{self, matrix4::Matrix4, vector3::Vector3},
};

pub struct AssetManager {
    textures: HashMap<String, Rc<Texture>>,
    sprites: Vec<Rc<RefCell<dyn SpriteComponent>>>,
    pub sprite_verts: VertexArray,
    pub sprite_shader: Shader,
    meshes: HashMap<String, Rc<Mesh>>,
    pub mesh_shader: Shader,
    mesh_components: Vec<Rc<RefCell<MeshComponent>>>,
}

impl AssetManager {
    pub fn new() -> Rc<RefCell<Self>> {
        let this = Self {
            textures: HashMap::new(),
            sprites: vec![],
            sprite_verts: Self::create_sprite_verts(),
            sprite_shader: Shader::new(),
            meshes: HashMap::new(),
            mesh_shader: Shader::new(),
            mesh_components: vec![],
        };

        Rc::new(RefCell::new(this))
    }

    fn create_sprite_verts() -> VertexArray {
        let vertices = [
            -0.5, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, // top left
            0.5, 0.5, 0.0, 0.0, 0.0, 1.0, 0.0, // top right
            0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 1.0, // bottom right
            -0.5, -0.5, 0.0, 0.0, 0.0, 0.0, 1.0, // bottom left
        ];

        let index_buffer = [
            0, 1, 2, // right half of triangle
            2, 3, 0, // left half of triangle
        ];

        VertexArray::new(&vertices, 4, &index_buffer, 6)
    }

    pub fn load_shaders(
        &mut self,
        screen_width: f32,
        screen_height: f32,
    ) -> Result<(Matrix4, Matrix4)> {
        self.sprite_shader.load("Sprite.vert", "Sprite.frag")?;
        self.sprite_shader.set_active();

        let view_proj = Matrix4::create_simple_view_proj(1024.0, 768.0);
        self.sprite_shader
            .set_matrix_uniform("uViewProj", view_proj);

        self.mesh_shader.load("Phong.vert", "Phong.frag")?;
        self.mesh_shader.set_active();

        let view = Matrix4::create_look_at(&Vector3::ZERO, &Vector3::UNIT_X, &Vector3::UNIT_Z);
        let projection = Matrix4::create_perspective_fov(
            math::basic::to_radians(70.0),
            screen_width,
            screen_height,
            25.0,
            10000.0,
        );
        self.mesh_shader
            .set_matrix_uniform("uViewProj", view.clone() * projection.clone());

        Ok((view, projection))
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

        self.get_default_texture()
    }

    pub fn get_default_texture(&mut self) -> Rc<Texture> {
        let file_name = "Default.png";
        if let Some(texture) = self.textures.get(&file_name.to_string()) {
            return texture.clone();
        }

        let mut texture = Texture::new();
        texture.load(file_name).unwrap();
        let result = Rc::new(texture);
        self.textures.insert(file_name.to_string(), result.clone());
        return result;
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

    pub fn get_mesh(&mut self, file_name: &str) -> Rc<Mesh> {
        if let Some(mesh) = self.meshes.get(&file_name.to_string()) {
            return mesh.clone();
        }

        let mut mesh = Mesh::new();
        if mesh.load(file_name, self).is_ok() {
            let result = Rc::new(mesh);
            self.meshes.insert(file_name.to_string(), result.clone());
            return result;
        }

        panic!()
    }

    pub fn add_mesh_component(&mut self, mesh: Rc<RefCell<MeshComponent>>) {
        self.mesh_components.push(mesh);
    }

    pub fn get_mesh_components(&self) -> &Vec<Rc<RefCell<MeshComponent>>> {
        &self.mesh_components
    }
}
