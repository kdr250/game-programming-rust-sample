use std::{ffi::CString, path::Path, ptr::null, ptr::null_mut};

use anyhow::{anyhow, Ok, Result};
use gl::{
    types::{GLenum, GLuint},
    COMPILE_STATUS, FRAGMENT_SHADER, LINK_STATUS, TRUE, VERTEX_SHADER,
};

use crate::math::matrix4::Matrix4;

pub struct Shader {
    // OpenGL IDs of the vertex shader
    vertex_shader: GLuint,

    // OpenGL IDs of the frag shader
    frag_shader: GLuint,

    // OpenGL IDs of the shader program
    shader_program: GLuint,
}

impl Shader {
    pub fn new() -> Self {
        Self {
            vertex_shader: 0,
            frag_shader: 0,
            shader_program: 0,
        }
    }

    pub fn load(&mut self, vert_name: &str, frag_name: &str) -> Result<()> {
        // Compile vertex and pixel shaders
        self.vertex_shader = self.compile_shader(vert_name, VERTEX_SHADER)?;
        self.frag_shader = self.compile_shader(frag_name, FRAGMENT_SHADER)?;

        // Now create a shader program that links together the vertex/frag shaders
        unsafe {
            self.shader_program = gl::CreateProgram();
            gl::AttachShader(self.shader_program, self.vertex_shader);
            gl::AttachShader(self.shader_program, self.frag_shader);
            gl::LinkProgram(self.shader_program);
        }

        self.is_valid_program()?;

        Ok(())
    }

    pub fn unload(&mut self) {
        unsafe {
            gl::DeleteProgram(self.shader_program);
            gl::DeleteShader(self.vertex_shader);
            gl::DeleteShader(self.frag_shader);
        }
    }

    pub fn set_active(&self) {
        unsafe {
            gl::UseProgram(self.shader_program);
        }
    }

    pub fn set_matrix_uniform(&self, name: &str, matrix: Matrix4) {
        unsafe {
            // Find the uniform by this name
            let name = CString::new(name).unwrap();
            let location_id = gl::GetUniformLocation(self.shader_program, name.as_ptr());
            // Send the matrix data to the uniform
            gl::UniformMatrix4fv(location_id, 1, TRUE, matrix.get_as_float_ptr());
        }
    }

    fn compile_shader(&mut self, file_name: &str, shader_type: GLenum) -> Result<u32> {
        // Read all the text into a string
        let path = Path::new(env!("OUT_DIR"))
            .join("resources")
            .join("Shaders")
            .join(file_name);
        let contents = std::fs::read_to_string(path)?;
        let contents = CString::new(contents.as_str())?;
        let contents_char = contents.as_ptr();
        let mut out_shader = 0;

        unsafe {
            // Create a shader of the specified type
            out_shader = gl::CreateShader(shader_type);

            // Set the source characters and try to compile
            gl::ShaderSource(out_shader, 1, &contents_char, null());
            gl::CompileShader(out_shader);
        }

        if let Err(error) = self.is_compiled(out_shader) {
            return Err(anyhow!("Failed to comple shader {}: {}", file_name, error));
        }

        Ok(out_shader)
    }

    fn is_compiled(&self, shader: u32) -> Result<()> {
        let mut status = 0;
        unsafe {
            gl::GetShaderiv(shader, COMPILE_STATUS, &mut status);
            if status != TRUE as i32 {
                let mut buffer = [0_i8; 512];
                gl::GetShaderInfoLog(shader, 511, null_mut(), buffer.as_mut_ptr());
                let buf = Vec::from(buffer).into_iter().map(|b| b as u8).collect();
                let error = String::from_utf8(buf)?;
                return Err(anyhow!("Failed to compile GLSL: {}", error));
            }
        }

        Ok(())
    }

    fn is_valid_program(&self) -> Result<()> {
        let mut status = 0;
        unsafe {
            gl::GetProgramiv(self.shader_program, LINK_STATUS, &mut status);
            if status != TRUE as i32 {
                let mut buffer = [0_i8; 512];
                gl::GetShaderInfoLog(self.shader_program, 511, null_mut(), buffer.as_mut_ptr());
                let buf = Vec::from(buffer).into_iter().map(|b| b as u8).collect();
                let error = String::from_utf8(buf)?;
                return Err(anyhow!("GLSL Link status: {}", error));
            }
        }

        Ok(())
    }
}
