use gl::types::*;
use std::mem;
pub trait GLSize: Sized {
    fn gl_size_of() -> usize;
    fn gl_type() -> GLenum;
}

impl GLSize for f32 {
    fn gl_size_of() -> usize {
        mem::size_of::<GLfloat>()
    }

    fn gl_type() -> GLenum {
        gl::FLOAT
    }
}

impl GLSize for bool {
    fn gl_size_of() -> usize {
        mem::size_of::<GLboolean>()
    }

    fn gl_type() -> GLenum {
        gl::BOOL
    }
}

impl GLSize for u32 {
    fn gl_size_of() -> usize {
        mem::size_of::<GLuint>()
    }

    fn gl_type() -> GLenum {
        gl::UNSIGNED_INT
    }
}

impl GLSize for i32 {
    fn gl_size_of() -> usize {
        mem::size_of::<GLint>()
    }

    fn gl_type() -> GLenum {
        gl::INT
    }
}
