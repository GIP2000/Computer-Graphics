pub mod bindable;
pub mod ebo;
pub mod texture;

use super::gl_size::GLSize;
use anyhow::{bail, Result};
use bindable::Bindable;
use gl::types::*;
use std::os::raw::c_void;

unsafe fn make_buffer<T: GLSize>(data: &[T], buffer_type: GLenum, usage: GLenum) -> Result<u32> {
    let mut buffer = 0;

    gl::GenBuffers(1, &mut buffer);
    if buffer <= 0 {
        bail!("Failed to make the VBO");
    }
    gl::BindBuffer(buffer_type, buffer);
    gl::BufferData(
        buffer_type,
        (data.len() * T::gl_size_of()) as GLsizeiptr,
        &data[0] as *const T as *const c_void,
        usage,
    );
    return Ok(buffer);
}

pub struct VOs {
    vbo: u32,
    vao: u32,
    shape: GLenum,
}

pub struct Attribute {
    pub location: u32,
    pub size: i32,
    pub normalized: bool,
    pub stride: usize,
    pub offset: usize,
}

impl Drop for VOs {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}

impl VOs {
    pub fn new<T: GLSize>(verts: &[T], attributes: &[Attribute], shape: GLenum) -> Result<Self> {
        let vbo;
        let mut vao = 0;
        if verts.len() <= 0 {
            bail!("input verts was empty");
        }
        if attributes.len() >= gl::MAX_VERTEX_ATTRIBS as usize {
            bail!("Too many attributes")
        }
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            if vao <= 0 {
                bail!("Failed to make the VBA");
            }
            gl::BindVertexArray(vao);
            vbo = make_buffer(verts, gl::ARRAY_BUFFER, gl::STATIC_DRAW)?;
            for (i, at) in attributes.iter().enumerate() {
                if at.size < 1 || at.size > 4 {
                    let _ = Self { vbo, vao, shape }; // this drops the current VBO and VAO
                                                      // deleting them from GPU memory
                    bail!("size is not between 1-4 or GLBGRA")
                }

                if at.offset >= verts.len() || at.stride >= verts.len() {
                    let _ = Self { vbo, vao, shape }; // this drops the current VBO and VAO
                                                      // deleting them from GPU memory
                    bail!("Properties do not make sense")
                }
                gl::VertexAttribPointer(
                    at.location,
                    at.size,
                    T::gl_type(),
                    if at.normalized { gl::TRUE } else { gl::FALSE },
                    (at.stride * T::gl_size_of()) as GLsizei,
                    (at.offset * T::gl_size_of()) as *const c_void,
                );
                gl::EnableVertexAttribArray(i as GLuint);
            }
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        };
        Ok(Self { vbo, vao, shape })
    }

    pub fn set_shape(&mut self, shape: GLenum) {
        self.shape = shape;
    }

    pub fn draw_arrays(&self, start: i32, len: u32) -> Result<()> {
        let len = len as i32;
        self.bind()?;
        unsafe {
            gl::DrawArrays(self.shape, start, len);
        }
        Ok(())
    }
}

impl Bindable for VOs {
    fn bind(&self) -> Result<()> {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        }
        Ok(())
    }
}
