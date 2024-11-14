use std::{cell::RefCell, ptr::null, rc::Rc};

use gl::{TRIANGLES, UNSIGNED_INT};

use crate::{
    actors::actor::Actor,
    graphics::{mesh::Mesh, shader::Shader},
    math::{matrix4::Matrix4, quaternion::Quaternion, vector3::Vector3},
};

use super::component::{self, generate_id, Component, State};

pub struct MeshComponent {
    id: u32,
    owner: Rc<RefCell<dyn Actor>>,
    update_order: i32,
    state: State,
    mesh: Option<Rc<Mesh>>,
    texture_index: usize,
}

impl MeshComponent {
    pub fn new(owner: Rc<RefCell<dyn Actor>>) -> Rc<RefCell<Self>> {
        let this = Self {
            id: generate_id(),
            owner: owner.clone(),
            update_order: 100,
            state: State::Active,
            mesh: None,
            texture_index: 0,
        };

        let result = Rc::new(RefCell::new(this));

        owner.borrow_mut().add_component(result.clone());
        owner
            .borrow_mut()
            .get_asset_manager()
            .borrow_mut()
            .add_mesh_component(result.clone());

        result
    }

    pub fn draw(&self, shader: &Shader) {
        if let Some(mesh) = &self.mesh {
            // Set the world transform
            shader.set_matrix_uniform(
                "uWorldTransform",
                self.owner.borrow().get_world_transform().clone(),
            );

            // Set specular power
            shader.set_float_uniform("uSpecPower", mesh.get_spec_power());

            // Set the active texture
            if let Some(texture) = mesh.get_texture(self.texture_index) {
                texture.set_active();
            }

            // Set the mesh's vertex array as active
            let vertex_array = mesh.get_vertex_array();
            vertex_array.set_active();

            unsafe {
                // Draw
                gl::DrawElements(
                    TRIANGLES,
                    vertex_array.get_num_indices() as i32,
                    UNSIGNED_INT,
                    null(),
                );
            }
        }
    }

    pub fn set_mesh(&mut self, mesh: Rc<Mesh>) {
        self.mesh = Some(mesh);
    }
}

impl Component for MeshComponent {
    fn update(
        &mut self,
        _delta_time: f32,
        _owner_info: &(Vector3, Quaternion, Vector3, Matrix4),
    ) -> (Option<Vector3>, Option<Quaternion>, Option<Vector3>) {
        (None, None, None)
    }

    component::impl_getters_setters! {}
}
