use std::{cell::RefCell, rc::Rc};

use anyhow::{anyhow, Result};
use gl::{BLEND, DEPTH_TEST, FUNC_ADD, ONE, ONE_MINUS_SRC_ALPHA, SRC_ALPHA, ZERO};
use sdl2::{
    video::{GLContext, Window},
    VideoSubsystem,
};

use crate::{
    graphics::{directional_light::DirectionalLight, shader::Shader},
    math::{matrix4::Matrix4, vector3::Vector3},
};

use super::asset_manager::AssetManager;

pub struct Renderer {
    asset_manager: Rc<RefCell<AssetManager>>,

    // View/projection for 3D shaders
    view: Matrix4,
    projection: Matrix4,

    // Width/height of screen
    screen_width: f32,
    screen_height: f32,

    // Lighting data
    ambient_light: Vector3,
    directional_light: DirectionalLight,

    // Window
    window: Window,

    // OpenGL context
    context: GLContext,
}

impl Renderer {
    pub fn initialize(
        video_system: VideoSubsystem,
        screen_width_height: (f32, f32),
    ) -> Result<Rc<RefCell<Self>>> {
        let (screen_width, screen_height) = screen_width_height;

        let gl_attr = video_system.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);
        gl_attr.set_red_size(8);
        gl_attr.set_green_size(8);
        gl_attr.set_blue_size(8);
        gl_attr.set_alpha_size(8);
        gl_attr.set_double_buffer(true);
        gl_attr.set_accelerated_visual(true);
        gl_attr.set_depth_size(24);

        let window = video_system
            .window("Rust Game", screen_width as u32, screen_height as u32)
            .position(100, 100)
            .opengl()
            .build()?;

        let context = window.gl_create_context().map_err(|e| anyhow!(e))?;
        gl::load_with(|name| video_system.gl_get_proc_address(name) as *const _);

        let asset_manager = AssetManager::new();
        let (view, projection) = asset_manager
            .borrow_mut()
            .load_shaders(screen_width_height.0, screen_width_height.1)?;

        let this = Self {
            asset_manager,
            view,
            projection,
            screen_width,
            screen_height,
            ambient_light: Vector3::ZERO,
            directional_light: DirectionalLight::new(),
            window,
            context,
        };

        Ok(Rc::new(RefCell::new(this)))
    }

    pub fn draw(&mut self) {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // Draw mesh components. Enable depth buffering/disable alpha blend
            gl::Enable(DEPTH_TEST);
            gl::Disable(BLEND);
        }

        // Set the mesh shader active
        let asset_manager = self.asset_manager.borrow_mut();
        asset_manager.mesh_shader.set_active();

        // Update view-projection matrix
        asset_manager
            .mesh_shader
            .set_matrix_uniform("uViewProj", self.view.clone() * self.projection.clone());

        // Draw mesh components
        let mesh_components = asset_manager.get_mesh_components().clone();
        for mesh_component in mesh_components {
            mesh_component.borrow().draw(&asset_manager.mesh_shader);
        }

        unsafe {
            // Draw all sprite components. Disable depth buffering
            gl::Disable(DEPTH_TEST);
            gl::Enable(BLEND);
            gl::BlendEquationSeparate(FUNC_ADD, FUNC_ADD);
            gl::BlendFuncSeparate(SRC_ALPHA, ONE_MINUS_SRC_ALPHA, ONE, ZERO);
        }

        // Set shader/vao as active
        asset_manager.sprite_shader.set_active();
        asset_manager.sprite_verts.set_active();

        for sprite in asset_manager.get_sprites() {
            sprite.borrow().draw(&asset_manager.sprite_shader);
        }

        // Swap the buffers
        self.window.gl_swap_window();
    }

    pub fn set_light_uniforms(&mut self, shader: &mut Shader) {
        // Camera position is from inverted view
        let mut inverted_view = self.view.clone();
        inverted_view.invert();
        shader.set_vector_uniform("uCameraPos", &inverted_view.get_translation());

        // Ambient light
        shader.set_vector_uniform("uAmbientLight", &self.ambient_light);

        // Directional light
        shader.set_vector_uniform("uDirLight.mDirection", &self.directional_light.direction);
        shader.set_vector_uniform(
            "uDirLight.mDiffuseColor",
            &self.directional_light.diffuse_color,
        );
        shader.set_vector_uniform("uDirLight.mSpecColor", &self.directional_light.spec_color);
    }

    pub fn get_asset_manager(&self) -> &Rc<RefCell<AssetManager>> {
        &self.asset_manager
    }

    pub fn set_view_matrix(&mut self, view: Matrix4) {
        self.view = view;
    }
}
