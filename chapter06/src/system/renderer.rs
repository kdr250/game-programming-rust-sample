use std::{cell::RefCell, rc::Rc};

use anyhow::{anyhow, Result};
use gl::{BLEND, ONE_MINUS_SRC_ALPHA, SRC_ALPHA};
use sdl2::{
    video::{GLContext, Window},
    VideoSubsystem,
};

use crate::math::matrix4::Matrix4;

use super::asset_manager::AssetManager;

pub struct Renderer {
    asset_manager: Rc<RefCell<AssetManager>>,

    // View/projection for 3D shaders
    view: Matrix4,
    projection: Matrix4,

    // Width/height of screen
    screen_width: f32,
    screen_height: f32,

    // Window
    window: Window,

    // OpenGL context
    context: GLContext,
}

impl Renderer {
    pub fn initialize(
        video_system: VideoSubsystem,
        screen_width_height: (f32, f32),
    ) -> Result<Self> {
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

        let window = video_system
            .window("Rust Game", screen_width as u32, screen_height as u32)
            .position(100, 100)
            .opengl()
            .build()?;

        let context = window.gl_create_context().map_err(|e| anyhow!(e))?;
        gl::load_with(|name| video_system.gl_get_proc_address(name) as *const _);

        let asset_manager = AssetManager::new();
        asset_manager.borrow_mut().load_shaders()?;

        Ok(Self {
            asset_manager,
            view: Matrix4::new(),
            projection: Matrix4::new(),
            screen_width,
            screen_height,
            window,
            context,
        })
    }

    pub fn draw(&mut self) {
        unsafe {
            gl::ClearColor(0.86, 0.86, 0.86, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        let assert_manager = self.asset_manager.borrow_mut();
        assert_manager.sprite_shader.set_active();
        assert_manager.sprite_verts.set_active();

        for sprite in assert_manager.get_sprites() {
            sprite.borrow().draw(&assert_manager.sprite_shader);
        }

        unsafe {
            gl::Enable(BLEND);
            gl::BlendFunc(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
        }

        self.window.gl_swap_window();
    }

    pub fn get_asset_manager(&self) -> &Rc<RefCell<AssetManager>> {
        &self.asset_manager
    }
}
