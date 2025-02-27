pub mod set_uniform;
use anyhow::{bail, Context, Result};
use gl::types::*;
use set_uniform::SetUniform;
use std::{ffi::CString, ptr};

pub struct Shader {
    shader: u32,
}

pub struct ShaderProgram {
    pub id: u32,
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

impl ShaderProgram {
    pub fn use_program(&self) {
        unsafe { gl::UseProgram(self.id) }
    }

    unsafe fn get_uniform(&self, name: &str) -> Result<i32> {
        Ok(gl::GetUniformLocation(
            self.id,
            CString::new(name.as_bytes())?.as_ptr(),
        ))
    }

    fn set_uniform_dyn(&self, name: &str, data: &dyn SetUniform) -> Result<()> {
        self.use_program();
        if !data.has_next() {
            unsafe {
                let uid = self.get_uniform(name)?;
                // if uid < 0 {
                //     println!("Invalid UID {} for name {}", uid, name);
                // }
                data.set_uniform(uid);
            }
            return Ok(());
        }
        for (name, d) in data.name_data_list(name) {
            self.set_uniform_dyn(&name, d)?;
        }
        return Ok(());
    }

    pub fn set_uniform<T: SetUniform>(&self, name: &str, data: T) -> Result<()> {
        self.use_program();
        if !data.has_next() {
            unsafe {
                let uid = self.get_uniform(name)?;
                // if uid < 0 {
                //     println!("Invalid UID {} for name {}", uid, name);
                // }
                // println!("setting name {name}");
                data.set_uniform(uid);
            }
            return Ok(());
        }
        for (name, d) in data.name_data_list(name) {
            self.set_uniform_dyn(&name, d)?;
        }
        return Ok(());
    }

    pub fn new<const N: usize>(shaders: [Shader; N]) -> Result<Self> {
        let id;
        unsafe {
            id = gl::CreateProgram();
            if id == 0 {
                bail!("Couldn't Create Shader Program")
            }
            for shader in shaders.iter() {
                gl::AttachShader(id, shader.shader);
            }
            gl::LinkProgram(id);
        }
        Ok(Self { id })
    }

    pub fn new_shallow(shaders: &[Shader]) -> Result<Self> {
        let id;
        unsafe {
            id = gl::CreateProgram();
            if id == 0 {
                bail!("Couldn't Create Shader Program")
            }
            for shader in shaders.iter() {
                gl::AttachShader(id, shader.shader);
            }
            gl::LinkProgram(id);
        }
        Ok(Self { id })
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.shader);
        }
    }
}

impl Shader {
    pub fn new(shader_str: &str, shader_type: gl::types::GLenum) -> Result<Self> {
        let shader;
        unsafe {
            shader = gl::CreateShader(shader_type);
            if shader == 0 {
                bail!("Error Creating Shader")
            }
            let c_shader = CString::new(shader_str.as_bytes())?;
            gl::ShaderSource(shader, 1, &c_shader.as_ptr(), ptr::null());
            gl::CompileShader(shader);

            let mut success = gl::FALSE as GLint;
            let mut info_log: Vec<u8> = Vec::with_capacity(1024);
            info_log.set_len(1024 - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(
                    shader,
                    1024,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                let info_log = info_log
                    .split(|&c| c == 0)
                    .next()
                    .context("Malformed Error message")?;
                bail!(
                    "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}",
                    std::str::from_utf8(info_log)?
                );
            }
        }
        Ok(Self { shader })
    }
}
