use cgmath::prelude::*;
use cgmath::Matrix4;
use cgmath::Vector3;
use cgmath::Vector4;

pub trait SetUniform {
    unsafe fn set_uniform(&self, id: i32);
}

impl SetUniform for bool {
    unsafe fn set_uniform(&self, id: i32) {
        gl::Uniform1i(id, *self as i32);
    }
}

impl SetUniform for i32 {
    unsafe fn set_uniform(&self, id: i32) {
        gl::Uniform1i(id, *self);
    }
}

impl SetUniform for f32 {
    unsafe fn set_uniform(&self, id: i32) {
        gl::Uniform1f(id, *self);
    }
}

impl SetUniform for Matrix4<f32> {
    unsafe fn set_uniform(&self, id: i32) {
        gl::UniformMatrix4fv(id, 1, gl::FALSE, self.as_ptr());
    }
}
impl SetUniform for Vec<Vector4<f32>> {
    unsafe fn set_uniform(&self, id: i32) {
        gl::Uniform4fv(id, self.len() as i32, self[0].as_ptr());
    }
}
impl SetUniform for Vector4<f32> {
    unsafe fn set_uniform(&self, id: i32) {
        gl::Uniform4fv(id, 1, self.as_ptr());
    }
}

impl SetUniform for Vector3<f32> {
    unsafe fn set_uniform(&self, id: i32) {
        gl::Uniform3fv(id, 1, self.as_ptr());
    }
}

impl SetUniform for (f32, f32, f32) {
    unsafe fn set_uniform(&self, id: i32) {
        gl::Uniform3f(id, self.0, self.1, self.2);
    }
}