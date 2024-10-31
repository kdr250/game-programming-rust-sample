use std::{cell::RefCell, path::Path, rc::Rc};

use anyhow::{anyhow, Ok, Result};
use serde_json::{json, Value};

use crate::{math::vector3::Vector3, system::asset_manager::AssetManager};

use super::{texture::Texture, vertex_array::VertexArray};

pub struct Mesh {
    textures: Vec<Rc<Texture>>,
    vertex_array: Option<Rc<VertexArray>>,
    shader_name: String,
    radius: f32,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            textures: vec![],
            vertex_array: None,
            shader_name: String::new(),
            radius: 0.0,
        }
    }

    pub fn load(&mut self, file_name: &str, asset_manager: &mut AssetManager) -> Result<()> {
        let path = Path::new(env!("OUT_DIR"))
            .join("resources")
            .join("Assets")
            .join(file_name);
        let content = std::fs::read_to_string(path)?;
        let json: Value = serde_json::from_str(&content)?;

        // Check the version
        let version = &json["version"].as_i64().unwrap();
        if *version != 1 {
            return Err(anyhow!("Mesh {} not version 1", file_name));
        }

        self.shader_name = json["shader"].as_str().unwrap().to_string();

        // Skip the vertex format/shader for now
        // (This is changed in a later chapter's code)
        let vert_size = 8;

        // Load textures
        let textures = &json["textures"];
        if !textures.is_array() || textures.as_array().unwrap().len() < 1 {
            return Err(anyhow!(
                "Mesh {} has no textures, there should be at least one",
                file_name
            ));
        }

        let textures = textures.as_array().unwrap();
        for i in 0..textures.len() {
            // Is this texture already loaded?
            let texture_name = textures[i].as_str().unwrap();
            let texture = asset_manager.get_texture(texture_name);
            self.textures.push(texture);
        }

        // Load in the vertices
        let verts_json = &json["vertices"];
        if !verts_json.is_array() || verts_json.as_array().unwrap().len() < 1 {
            return Err(anyhow!("Mesh {} has no vertices", file_name));
        }

        let verts_json = verts_json.as_array().unwrap();
        let mut vertices = vec![];
        for i in 0..verts_json.len() {
            // For now, just assume we have 8 elements
            let vert = &verts_json[i];
            if !vert.is_array() || vert.as_array().unwrap().len() != 8 {
                return Err(anyhow!("Unexpected vertex format for {}", file_name));
            }

            let vert = vert.as_array().unwrap();

            let position = Vector3::new(
                vert[0].as_f64().unwrap() as f32,
                vert[1].as_f64().unwrap() as f32,
                vert[2].as_f64().unwrap() as f32,
            );
            self.radius = self.radius.max(position.length_sqrt());

            // Add the floats
            for i in 0..vert.len() {
                vertices.push(vert[i].as_f64().unwrap() as f32);
            }
        }

        // We were computing length squared earlier
        self.radius = self.radius.sqrt();

        // Load in the indices
        let ind_json = &json["indices"];
        if !ind_json.is_array() || ind_json.as_array().unwrap().len() < 1 {
            return Err(anyhow!("Mesh {} has no indices", file_name));
        }

        let ind_json = ind_json.as_array().unwrap();
        let mut indices = vec![];
        for i in 0..ind_json.len() {
            let ind = &ind_json[i];
            if !ind.is_array() || ind.as_array().unwrap().len() != 3 {
                return Err(anyhow!("Invalid indices for {}", file_name));
            }

            let ind = ind.as_array().unwrap();
            indices.push(ind[0].as_u64().unwrap() as u32);
            indices.push(ind[1].as_u64().unwrap() as u32);
            indices.push(ind[2].as_u64().unwrap() as u32);
        }

        // Now create a vertex array
        let vertex_array = VertexArray::new(
            &vertices,
            (vertices.len() / vert_size) as isize,
            &indices,
            indices.len() as isize,
        );

        self.vertex_array = Some(Rc::new(vertex_array));

        Ok(())
    }

    pub fn get_vertex_array(&self) -> Rc<VertexArray> {
        self.vertex_array.clone().unwrap()
    }

    pub fn get_texture(&self, index: usize) -> Option<Rc<Texture>> {
        self.textures.get(index).cloned()
    }

    pub fn get_shader_name(&self) -> &String {
        &self.shader_name
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }
}
