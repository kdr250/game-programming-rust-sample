use std::{os::raw::c_void, path::Path};

use anyhow::{Ok, Result};
use gl::{LINEAR, RGB, RGBA, TEXTURE_2D, TEXTURE_MAG_FILTER, TEXTURE_MIN_FILTER, UNSIGNED_BYTE};
use image::{ColorType, ImageReader};

pub struct Texture {
    // OpenGL ID of this texture
    texture_id: u32,

    // Width/height of the texture
    width: i32,
    height: i32,
}

impl Texture {
    pub fn new() -> Self {
        Self {
            texture_id: 0,
            width: 0,
            height: 0,
        }
    }

    pub fn load(&mut self, file_name: &str) -> Result<()> {
        let path = Path::new(env!("OUT_DIR"))
            .join("resources")
            .join("Assets")
            .join(file_name);

        let image = ImageReader::open(path)?.decode()?;

        self.width = image.width() as i32;
        self.height = image.height() as i32;

        let format = match image.color() {
            ColorType::Rgba8 | ColorType::Rgba16 | ColorType::Rgba32F => RGBA,
            _ => RGB,
        };

        unsafe {
            gl::GenTextures(1, &mut self.texture_id);
            gl::BindTexture(TEXTURE_2D, self.texture_id);

            gl::TexImage2D(
                TEXTURE_2D,
                0,
                format as i32,
                self.width,
                self.height,
                0,
                format,
                UNSIGNED_BYTE,
                image.as_bytes().as_ptr() as *const c_void,
            );

            // Enable bilinear filtering
            gl::TexParameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as i32);
            gl::TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);
        }

        Ok(())
    }

    pub fn unload(&self) {
        unsafe {
            gl::DeleteTextures(1, &self.texture_id);
        }
    }

    pub fn set_active(&self) {
        unsafe {
            gl::BindTexture(TEXTURE_2D, self.texture_id);
        }
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }
}
