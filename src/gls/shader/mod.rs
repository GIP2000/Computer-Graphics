mod set_uniform;
use anyhow::{bail, Result};
use gl::types::*;
use set_uniform::SetUniform;
use std::{ffi::CString, ptr};

pub struct Shader {
    shader: u32,
}

pub struct ShaderProgram {
    id: u32,
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        println!("Deleteing Shader Program");
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

    pub fn set_uniform<T: SetUniform>(&self, name: &str, data: T) -> Result<()> {
        unsafe {
            let uid = self.get_uniform(name)?;
            if uid <= 0 {
                bail!("Invalid UID")
            }
            data.set_uniform(uid);
        }
        Ok(())
    }

    pub fn new<const N: usize>(shaders: [Shader; N]) -> Result<Self> {
        println!("Linking and Building Program");
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
        println!("Dropping Shader");
        unsafe {
            gl::DeleteShader(self.shader);
        }
    }
}

impl Shader {
    pub fn new(shader_str: &str, shader_type: gl::types::GLenum) -> Result<Self> {
        println!("Building Shader");
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
            let mut info_log: Vec<char> = Vec::with_capacity(512);
            info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(
                    shader,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                bail!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{:?}", info_log);
            }
        }
        Ok(Self { shader })
    }
}
