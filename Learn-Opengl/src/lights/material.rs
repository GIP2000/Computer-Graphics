use crate::gls::shader::set_uniform::SetUniform;

#[derive(Debug)]
pub struct Material {
    diffuse: i32,
    specular: i32,
    shininess: f32,
}

impl Material {
    pub fn new(diffuse: i32, specular: i32, shininess: f32) -> Self {
        Self {
            diffuse,
            specular,
            shininess,
        }
    }
}

impl SetUniform for Material {
    unsafe fn set_uniform(&self, id: i32) {
        panic!("can't set nested types");
    }

    fn name_data_list<'a>(&'a self, name: &str) -> Vec<(String, &'a dyn SetUniform)> {
        vec![
            (format!("{}.diffuse", name), &self.diffuse),
            (format!("{}.specular", name), &self.specular),
            (format!("{}.shininess", name), &self.shininess),
        ]
    }

    fn has_next(&self) -> bool {
        true
    }
}
