use super::bindable::Bindable;
use super::{make_buffer, GLSize, VOs};
use anyhow::Result;
use std::ffi::c_void;
use std::marker::PhantomData;

pub struct EBO<T: GLSize> {
    ebo: u32,
    phantom: PhantomData<T>,
}
impl<T: GLSize> Drop for EBO<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.ebo);
        }
    }
}

impl<T: GLSize> Bindable for EBO<T> {
    fn bind(&self) -> Result<()> {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
        };
        Ok(())
    }
}

impl<T: GLSize> EBO<T> {
    pub fn new(indices: &[T]) -> Result<Self> {
        let ebo = unsafe { make_buffer(indices, gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW)? };
        return Ok(Self {
            ebo,
            phantom: PhantomData,
        });
    }

    pub fn draw_elements(&self, vo: &VOs, count: u32, offset: usize) -> Result<()> {
        vo.bind()?;
        self.bind()?;
        unsafe {
            gl::DrawElements(
                vo.shape,
                count as i32,
                T::gl_type(),
                (offset * T::gl_size_of()) as *const c_void,
            )
        }
        return Ok(());
    }
}
