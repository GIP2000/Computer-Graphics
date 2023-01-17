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

impl SetUniform for Vec<f32> {
    unsafe fn set_uniform(&self, id: i32) {
        gl::Uniform3fv(id, 1, self.as_ptr());
    }
}

impl SetUniform for (f32, f32, f32) {
    unsafe fn set_uniform(&self, id: i32) {
        gl::Uniform3f(id, self.0, self.1, self.2);
    }
}
