use super::Bindable;
use anyhow::{bail, Context, Result};
use gl::types::*;
use std::ffi::c_void;

pub struct Textures<'a, const N: usize> {
    textures: [&'a Texture2D; N],
}

impl<'a, const N: usize> Bindable for Textures<'a, N> {
    fn bind(&self) {
        for (i, texture) in self.textures.iter().enumerate() {
            unsafe {
                gl::ActiveTexture(gl::TEXTURE0 + i as u32);
            }
            texture.bind();
        }
    }
}

impl<'a, const N: usize> Textures<'a, N> {
    pub fn new(textures: [&'a Texture2D; N]) -> Result<Self> {
        let mut max = 0;
        unsafe {
            gl::GetIntegerv(gl::MAX_COMBINED_TEXTURE_IMAGE_UNITS, &mut max);
        };
        if textures.len() as i32 >= max {
            bail!("Too many textures attempted")
        }
        Ok(Self { textures })
    }
}

#[derive(Debug)]
pub struct Texture2D {
    tex: u32,
}

impl Bindable for Texture2D {
    fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.tex);
        }
    }
}

impl Drop for Texture2D {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.tex);
        }
    }
}

impl Texture2D {
    pub fn new(
        img: image::DynamicImage,
        [x, y]: [GLenum; 2],
        [min, max]: [GLenum; 2],
        source_type: GLenum,
        border: Option<&[f32]>,
    ) -> Result<Self> {
        let data_ptr = if source_type == gl::RGB {
            img.as_rgb8().context("Couldn't convert to rgb")?.as_ptr()
        } else if source_type == gl::RGBA {
            img.as_rgba8().context("Couldn't convert to rgba")?.as_ptr()
        } else {
            bail!("Unsopported source image type")
        };
        // let data_ptr = img.as_rgb8().context("Couldn't convert to rgb")?.as_ptr();
        let mut tex = 0;

        unsafe {
            gl::GenTextures(1, &mut tex);
            if tex <= 0 {
                bail!("Couldn't Generate texture Buffer")
            }
            gl::BindTexture(gl::TEXTURE_2D, tex);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, x as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, y as i32);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, min as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, max as i32);

            if let Some(border) = border {
                gl::TexParameterfv(
                    gl::TEXTURE_2D,
                    gl::TEXTURE_BORDER_COLOR,
                    &border[0] as *const f32,
                )
            }

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                source_type,
                gl::UNSIGNED_BYTE,
                data_ptr as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Ok(Self { tex })
    }
}
