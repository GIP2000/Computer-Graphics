use super::gl_size::GLSize;
use anyhow::{bail, Result};
use gl::types::*;
use std::{marker::PhantomData, os::raw::c_void};

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
    vao: u32,
    vbo: u32,
    shape: GLenum,
}

pub struct EBO<T: GLSize> {
    ebo: u32,
    phantom: PhantomData<T>,
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
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            if vao <= 0 {
                bail!("Failed to make the VBA");
            }
            gl::BindVertexArray(vao);
            vbo = make_buffer(verts, gl::ARRAY_BUFFER, gl::STATIC_DRAW)?;
            for (i, at) in attributes.iter().enumerate() {
                if at.offset >= verts.len() // offset overflows
                    || (verts.len() - at.offset) % at.stride != 0
                // checks if stride will overflow
                {
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
                gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                gl::BindVertexArray(0);
            }
        };
        Ok(Self { vbo, vao, shape })
    }

    pub fn set_shape(&mut self, shape: GLenum) {
        self.shape = shape;
    }

    pub fn draw_arrays(&self, start: i32, len: i32) {
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
impl<T: GLSize> Drop for EBO<T> {
    fn drop(&mut self) {
        println!("Dropping EBO");
        unsafe {
            gl::DeleteBuffers(1, &self.ebo);
        }
    }
}

impl<T: GLSize> Bindable for EBO<T> {
    fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
        };
    }
}

impl<T: GLSize> EBO<T> {
    pub fn new(indices: &[T]) -> Result<Self> {
        println!("Building EBO");
        let ebo = unsafe { make_buffer(indices, gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW)? };
        return Ok(Self {
            ebo,
            phantom: PhantomData,
        });
    }

    pub fn draw_elements(&self, vo: &VOs, count: i32, offset: usize) {
        vo.bind();
        self.bind();
        unsafe {
            gl::DrawElements(
                vo.shape,
                count,
                T::gl_type(),
                (offset * T::gl_size_of()) as *const c_void,
            )
        }
    }
}
