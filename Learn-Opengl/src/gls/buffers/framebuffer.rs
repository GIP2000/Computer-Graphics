use anyhow::Result;

use super::{bindable::Bindable, texture::Tex2DTrait};
#[derive(Debug)]
pub struct FrameBuffer {
    fbo: u32,
}

impl FrameBuffer {
    pub fn new() -> Self {
        let mut fbo = 0;
        unsafe {
            gl::GenFramebuffers(1, &mut fbo);
        }
        Self { fbo }
    }

    pub fn unbind(&self) -> Result<()> {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
        Ok(())
    }
    pub fn drr(&self) -> Result<()> {
        self.bind()?;
        unsafe {
            gl::DrawBuffer(gl::NONE);
        }
        unsafe {
            gl::ReadBuffer(gl::NONE);
        }

        return Ok(());
    }
    pub fn draw(&self) -> Result<()> {
        self.bind()?;
        unsafe {
            gl::DrawBuffer(gl::NONE);
        }
        return Ok(());
    }

    pub fn read(&self) -> Result<()> {
        self.bind()?;
        unsafe {
            gl::ReadBuffer(gl::NONE);
        }
        return Ok(());
    }

    pub fn attach_tex(
        &self,
        texture: &dyn Tex2DTrait,
        attachment: gl::types::GLenum,
    ) -> Result<()> {
        self.bind()?;
        // texture.bind()?;
        unsafe {
            gl::FramebufferTexture(gl::FRAMEBUFFER, attachment, *texture.get_tex(), 0);
        }
        Ok(())
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteFramebuffers(1, &self.fbo) }
    }
}

impl Bindable for FrameBuffer {
    fn bind(&self) -> Result<()> {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);
        }
        Ok(())
    }
}
