use anyhow::Result;
use cgmath::{perspective, vec3, Deg, EuclideanSpace, Matrix4, Point3, Vector3};

use crate::gls::{
    buffers::{framebuffer::FrameBuffer, texture::CubeMap},
    shader::set_uniform::SetUniform,
};

#[derive(Debug)]
pub struct PointLight {
    position: Vector3<f32>,

    color: Vector3<f32>,

    constant: f32,
    linear: f32,
    quadratic: f32,

    ambient: Vector3<f32>,
    diffuse: Vector3<f32>,
    specular: Vector3<f32>,

    depth_map: i32,

    fbo: FrameBuffer,
    cube_map: CubeMap,
}

impl PointLight {
    fn new(
        position: Vector3<f32>,
        color: Vector3<f32>,
        constant: f32,
        linear: f32,
        quadratic: f32,
        ambient: Vector3<f32>,
        diffuse: Vector3<f32>,
        specular: Vector3<f32>,
        depth_map: i32,
    ) -> Result<Self> {
        let fbo = FrameBuffer::new();
        let cube_map = CubeMap::new(1024, 1024, gl::DEPTH_COMPONENT)?;
        fbo.attach_tex(&cube_map, gl::DEPTH_ATTACHMENT)?;
        fbo.drr()?;
        fbo.unbind()?;
        Ok(Self {
            position,
            color,
            constant,
            linear,
            quadratic,
            ambient,
            diffuse,
            specular,
            depth_map,
            fbo,
            cube_map,
        })
    }

    pub fn get_fbo(&self) -> &FrameBuffer {
        &self.fbo
    }

    pub fn get_cube_map(&self) -> &CubeMap {
        &self.cube_map
    }
    pub fn get_pos(&self) -> Vector3<f32> {
        self.position.clone()
    }

    pub fn get_shadow_proj(
        &self,
        width: f32,
        height: f32,
        near_plane: f32,
        far_plane: f32,
    ) -> Vec<Matrix4<f32>> {
        let shadow_proj = perspective(Deg(90.), width / height, near_plane, far_plane);

        return vec![
            shadow_proj
                * Matrix4::look_at_rh(
                    Point3::from_vec(self.get_pos()),
                    Point3::from_vec(self.get_pos() + vec3(1., 0., 0.)),
                    vec3(0., -1., 0.),
                ),
            shadow_proj
                * Matrix4::look_at_rh(
                    Point3::from_vec(self.get_pos()),
                    Point3::from_vec(self.get_pos() + vec3(-1., 0., 0.)),
                    vec3(0., -1., 0.),
                ),
            shadow_proj
                * Matrix4::look_at_rh(
                    Point3::from_vec(self.get_pos()),
                    Point3::from_vec(self.get_pos() + vec3(0., 1., 0.)),
                    vec3(0., 0., 1.),
                ),
            shadow_proj
                * Matrix4::look_at_rh(
                    Point3::from_vec(self.get_pos()),
                    Point3::from_vec(self.get_pos() + vec3(0., -1., 0.)),
                    vec3(0., 0., -1.),
                ),
            shadow_proj
                * Matrix4::look_at_rh(
                    Point3::from_vec(self.get_pos()),
                    Point3::from_vec(self.get_pos() + vec3(0., 0., 1.)),
                    vec3(0., -1., 0.),
                ),
            shadow_proj
                * Matrix4::look_at_rh(
                    Point3::from_vec(self.get_pos()),
                    Point3::from_vec(self.get_pos() + vec3(0., 0., -1.)),
                    vec3(0., -1., 0.),
                ),
        ];
    }
}
impl SetUniform for &Vec<PointLight> {
    unsafe fn set_uniform(&self, _id: i32) {
        panic!("Can't set a struct uniform directly");
    }

    fn name_data_list<'a>(&'a self, name: &str) -> Vec<(String, &'a dyn SetUniform)> {
        (0..10)
            .flat_map(|i| {
                let name = format!("{}[{}]", name, i);
                self[i.min(self.len() - 1)].name_data_list(name.as_str())
            })
            .collect::<Vec<(String, &dyn SetUniform)>>()
    }

    fn has_next(&self) -> bool {
        true
    }
}

impl SetUniform for PointLight {
    unsafe fn set_uniform(&self, _id: i32) {
        panic!("Can't set a struct uniform directly");
    }
    fn has_next(&self) -> bool {
        true
    }

    fn name_data_list<'a>(&'a self, name: &str) -> Vec<(String, &'a dyn SetUniform)> {
        vec![
            (format!("{name}.position"), &self.position),
            (format!("{name}.color"), &self.color),
            (format!("{name}.constant"), &self.constant),
            (format!("{name}.linear"), &self.linear),
            (format!("{name}.quadratic"), &self.quadratic),
            (format!("{name}.ambient"), &self.ambient),
            (format!("{name}.diffuse"), &self.diffuse),
            (format!("{name}.specular"), &self.specular),
            (format!("{name}.depthMap"), &self.depth_map),
        ]
    }
}

impl SetUniform for &PointLight {
    unsafe fn set_uniform(&self, _id: i32) {
        panic!("Can't set a struct uniform directly");
    }
    fn has_next(&self) -> bool {
        true
    }

    fn name_data_list<'a>(&'a self, name: &str) -> Vec<(String, &'a dyn SetUniform)> {
        vec![
            (format!("{name}.position"), &self.position),
            (format!("{name}.color"), &self.color),
            (format!("{name}.constant"), &self.constant),
            (format!("{name}.linear"), &self.linear),
            (format!("{name}.quadratic"), &self.quadratic),
            (format!("{name}.ambient"), &self.ambient),
            (format!("{name}.diffuse"), &self.diffuse),
            (format!("{name}.specular"), &self.specular),
            (format!("{name}.depthMap"), &self.depth_map),
        ]
    }
}

#[derive(Default, Clone)]
pub struct PointLightBuilder {
    pos: Option<Vector3<f32>>,

    color: Option<Vector3<f32>>,

    constant: Option<f32>,
    linear: Option<f32>,
    qudratic: Option<f32>,

    ambient: Option<Vector3<f32>>,
    diffuse: Option<Vector3<f32>>,
    specular: Option<Vector3<f32>>,

    depth_map: Option<i32>,
}

impl PointLightBuilder {
    pub fn build(&self) -> Result<PointLight> {
        PointLight::new(
            self.pos.unwrap_or(vec3(0., 0., 0.)),
            self.color.unwrap_or(vec3(1., 1., 1.)),
            self.constant.unwrap_or(1.),
            self.linear.unwrap_or(0.7),
            self.qudratic.unwrap_or(0.9),
            self.ambient.unwrap_or(vec3(0.3, 0.3, 0.3)),
            self.diffuse.unwrap_or(vec3(0.8, 0.8, 0.8)),
            self.specular.unwrap_or(vec3(0.3, 0.3, 0.3)),
            self.depth_map.unwrap_or(1),
        )
    }

    pub fn depth_map(mut self, depth_map: i32) -> Self {
        self.depth_map = Some(depth_map);
        self
    }

    pub fn pos(mut self, pos: Vector3<f32>) -> Self {
        self.pos = Some(pos);
        self
    }

    pub fn color(mut self, color: Vector3<f32>) -> Self {
        self.color = Some(color);
        self
    }

    pub fn ambient(mut self, ambient: Vector3<f32>) -> Self {
        self.ambient = Some(ambient);
        self
    }

    pub fn diffuse(mut self, diffuse: Vector3<f32>) -> Self {
        self.diffuse = Some(diffuse);
        self
    }

    pub fn specular(mut self, specular: Vector3<f32>) -> Self {
        self.specular = Some(specular);
        self
    }

    pub fn constant(mut self, constant: f32) -> Self {
        self.constant = Some(constant);
        self
    }

    pub fn linear(mut self, linear: f32) -> Self {
        self.linear = Some(linear);
        self
    }

    pub fn qudratic(mut self, qudratic: f32) -> Self {
        self.qudratic = Some(qudratic);
        self
    }
}
