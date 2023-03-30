use cgmath::{vec3, Vector3};

use crate::gls::shader::set_uniform::SetUniform;

pub struct PointLight {
    position: Vector3<f32>,

    constant: f32,
    linear: f32,
    qudratic: f32,

    ambient: Vector3<f32>,
    diffuse: Vector3<f32>,
    specular: Vector3<f32>,
}

impl PointLight {
    fn new(
        pos: Vector3<f32>,
        constant: f32,
        linear: f32,
        qudratic: f32,
        ambient: Vector3<f32>,
        diffuse: Vector3<f32>,
        specular: Vector3<f32>,
    ) -> Self {
        Self {
            position: pos,
            constant,
            linear,
            qudratic,
            ambient,
            diffuse,
            specular,
        }
    }
    pub fn get_pos(&self) -> Vector3<f32> {
        self.position.clone()
    }
}
impl SetUniform for &PointLight {
    unsafe fn set_uniform(&self, _id: i32) {
        panic!("Can't set a struct uniform directly");
    }
    fn has_next(&self) -> bool {
        true
    }

    fn name_data_list<'a>(&'a self, name: &'a str) -> Vec<(String, &'a dyn SetUniform)> {
        vec![
            (format!("{name}.position"), &self.position),
            (format!("{name}.constant"), &self.constant),
            (format!("{name}.linear"), &self.linear),
            (format!("{name}.qudratic"), &self.qudratic),
            (format!("{name}.ambient"), &self.ambient),
            (format!("{name}.diffuse"), &self.diffuse),
            (format!("{name}.specular"), &self.specular),
        ]
    }
}

#[derive(Default, Clone)]
pub struct PointLightBuilder {
    pos: Option<Vector3<f32>>,

    constant: Option<f32>,
    linear: Option<f32>,
    qudratic: Option<f32>,

    ambient: Option<Vector3<f32>>,
    diffuse: Option<Vector3<f32>>,
    specular: Option<Vector3<f32>>,
}

impl PointLightBuilder {
    pub fn build(&self) -> PointLight {
        PointLight::new(
            self.pos.unwrap_or(vec3(0., 0., 0.)),
            self.constant.unwrap_or(1.),
            self.linear.unwrap_or(0.7),
            self.qudratic.unwrap_or(1.8),
            self.ambient.unwrap_or(vec3(0.5, 0.5, 0.5)),
            self.diffuse.unwrap_or(vec3(0.8, 0.8, 0.8)),
            self.specular.unwrap_or(vec3(1., 1., 1.)),
        )
    }

    pub fn pos(&mut self, pos: Vector3<f32>) -> Self {
        self.pos = Some(pos);
        self.clone()
    }

    pub fn ambient(&mut self, ambient: Vector3<f32>) -> Self {
        self.ambient = Some(ambient);
        self.clone()
    }

    pub fn diffuse(&mut self, diffuse: Vector3<f32>) -> Self {
        self.diffuse = Some(diffuse);
        self.clone()
    }

    pub fn specular(&mut self, specular: Vector3<f32>) -> Self {
        self.specular = Some(specular);
        self.clone()
    }

    pub fn constant(&mut self, constant: f32) -> Self {
        self.constant = Some(constant);
        self.clone()
    }

    pub fn linear(&mut self, linear: f32) -> Self {
        self.linear = Some(linear);
        self.clone()
    }

    pub fn qudratic(&mut self, qudratic: f32) -> Self {
        self.qudratic = Some(qudratic);
        self.clone()
    }
}
