pub mod ebo;
pub mod texture;
use super::gl_size::GLSize;
use anyhow::{bail, Result};
use gl::types::*;
use std::os::raw::c_void;

pub trait Bindable {
    fn bind(&self);
}

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
    pub vao: u32,
    pub vbo: u32,
    pub shape: GLenum,
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
        println!("Dropping VOs");
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}

impl VOs {
    pub fn new<T: GLSize>(verts: &[T], attributes: &[Attribute], shape: GLenum) -> Result<Self> {
        println!("Building VBOAP");
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

    pub fn draw_arrays(&self, start: i32, len: u32) {
        let len = len as i32;
        self.bind();
        unsafe {
            gl::DrawArrays(self.shape, start, len);
        }
    }
}

impl Bindable for VOs {
    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        }
    }
}
