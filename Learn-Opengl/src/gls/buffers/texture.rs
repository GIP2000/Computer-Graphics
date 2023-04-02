use super::bindable::Bindable;
use anyhow::{bail, Context, Result};
use gl::types::GLenum;
use std::{ffi::c_void, ptr};

pub struct Textures<'a, const N: usize> {
    textures: [&'a dyn Tex2DTrait; N],
}

impl<'a, const N: usize> Bindable for Textures<'a, N> {
    fn bind(&self) -> Result<()> {
        for (i, texture) in self.textures.iter().enumerate() {
            unsafe {
                gl::ActiveTexture(gl::TEXTURE0 + i as u32);
            };
            texture.bind()?;
        }
        Ok(())
    }
}

impl<'a, const N: usize> Textures<'a, N> {
    pub fn new(textures: [&'a dyn Tex2DTrait; N]) -> Result<Self> {
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

pub trait Tex2DTrait: Bindable {
    fn get_tex(&self) -> &u32;
    fn drop_impl(&self) {
        unsafe {
            gl::DeleteTextures(1, self.get_tex());
        }
    }
}

#[derive(Debug)]
pub struct CubeMap {
    tex: u32,
}

impl CubeMap {
    pub fn new(width: i32, height: i32, format: gl::types::GLenum) -> Result<Self> {
        let mut tex = 0;
        unsafe {
            gl::GenTextures(1, &mut tex);
        }
        let tex = Self { tex };
        tex.bind()?;
        for i in 0..6 {
            unsafe {
                gl::TexImage2D(
                    gl::TEXTURE_CUBE_MAP_POSITIVE_X + i,
                    0,
                    format as i32,
                    width,
                    height,
                    0,
                    format,
                    gl::FLOAT,
                    ptr::null(),
                );
            }
        }
        unsafe {
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_MAG_FILTER,
                gl::NEAREST as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_MIN_FILTER,
                gl::NEAREST as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_EDGE as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_EDGE as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_WRAP_R,
                gl::CLAMP_TO_EDGE as i32,
            );
        }

        return Ok(tex);
    }
}

impl Tex2DTrait for CubeMap {
    fn get_tex(&self) -> &u32 {
        &self.tex
    }
}

impl Bindable for CubeMap {
    fn bind(&self) -> Result<()> {
        unsafe {
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, self.tex);
        }
        return Ok(());
    }
}

impl Drop for CubeMap {
    fn drop(&mut self) {
        return self.drop_impl();
    }
}

#[derive(Debug)]
pub struct Texture2D {
    tex: u32,
}

impl Bindable for Texture2D {
    fn bind(&self) -> Result<()> {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.tex);
        }
        return Ok(());
    }
}

impl Drop for Texture2D {
    fn drop(&mut self) {
        return self.drop_impl();
    }
}

impl Tex2DTrait for Texture2D {
    fn get_tex(&self) -> &u32 {
        &self.tex
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
        let mut tex = 0;
        unsafe {
            gl::GenTextures(1, &mut tex);
        }
        let tex = Self { tex };

        tex.bind().unwrap();

        unsafe {
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

        return Ok(tex);
    }
}
