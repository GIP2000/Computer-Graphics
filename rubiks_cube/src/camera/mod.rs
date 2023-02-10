use cgmath::{vec2, vec3, InnerSpace, Matrix4, MetricSpace, Point3, Vector2, Vector3};

const STEP: f32 = 0.05;

pub struct Camera {
    pos: Point3<f32>,
    look_at: Point3<f32>,
    radius: f32,
    last: Vector2<f32>,
}

impl Camera {
    pub fn new(pos: Point3<f32>, look_at: Point3<f32>) -> Self {
        Self {
            pos,
            look_at,
            radius: pos.distance(look_at),
            last: vec2(0., 0.),
        }
    }

    pub fn pan(&mut self, x: f32, y: f32) {
        let pos = vec2(x, y);
        let dir = (pos - self.last).normalize();
        let c = dir.dot(vec2(0., 0.));
        let s = 1. - (c * c);
    }

    pub fn get_view(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(self.pos, self.look_at, vec3(0., 1., 0.))
    }
}
