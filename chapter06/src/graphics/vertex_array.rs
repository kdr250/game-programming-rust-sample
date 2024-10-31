use std::os::raw::c_void;

use gl::{ARRAY_BUFFER, ELEMENT_ARRAY_BUFFER, FALSE, FLOAT, STATIC_DRAW};

pub struct VertexArray {
    // How many vertices in the vertex buffer?
    num_verts: isize,

    // How many indices in the index buffer
    num_indices: isize,

    // OpenGL ID of the vertex buffer
    vertex_buffer: u32,

    // OpenGL ID of the index buffer
    index_buffer: u32,

    // OpenGL ID of the vertex array object
    vertex_array: u32,
}

impl VertexArray {
    pub fn new(verts: &[f32], num_verts: isize, indices: &[u32], num_indices: isize) -> Self {
        let verts = verts.as_ptr();
        let indices = indices.as_ptr();
        let mut vertex_array = 0;
        let mut vertex_buffer = 0;
        let mut index_buffer = 0;

        unsafe {
            // Create vertex array
            gl::GenVertexArrays(1, &mut vertex_array);
            gl::BindVertexArray(vertex_array);

            // Create vertex buffer
            gl::GenBuffers(1, &mut vertex_buffer);
            gl::BindBuffer(ARRAY_BUFFER, vertex_buffer);
            gl::BufferData(
                ARRAY_BUFFER,
                num_verts * 5 * size_of::<f32>() as isize,
                verts as *const c_void,
                STATIC_DRAW,
            );

            // Create index buffer
            gl::GenBuffers(1, &mut index_buffer);
            gl::BindBuffer(ELEMENT_ARRAY_BUFFER, index_buffer);
            gl::BufferData(
                ELEMENT_ARRAY_BUFFER,
                num_indices * size_of::<u32>() as isize,
                indices as *const c_void,
                STATIC_DRAW,
            );

            // Specify the vertex attributes (For now, assume one vertex format)
            // Position is 3 floats
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                FLOAT,
                FALSE,
                size_of::<f32>() as i32 * 8,
                0 as *const c_void,
            );

            // Normal is 3 floats
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                3,
                FLOAT,
                FALSE,
                size_of::<f32>() as i32 * 8,
                (size_of::<f32>() * 3) as *const c_void,
            );

            // Texture coordinate is 2 floats
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                2,
                FLOAT,
                FALSE,
                size_of::<f32>() as i32 * 8,
                (size_of::<f32>() * 6) as *const c_void,
            );
        }

        Self {
            num_verts,
            num_indices,
            vertex_buffer,
            index_buffer,
            vertex_array,
        }
    }

    pub fn set_active(&self) {
        unsafe {
            gl::BindVertexArray(self.vertex_array);
        }
    }

    pub fn get_num_verts(&self) -> isize {
        self.num_verts
    }

    pub fn get_num_indices(&self) -> isize {
        self.num_indices
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vertex_buffer);
            gl::DeleteBuffers(1, &self.index_buffer);
            gl::DeleteVertexArrays(1, &self.vertex_array);
        }
    }
}
