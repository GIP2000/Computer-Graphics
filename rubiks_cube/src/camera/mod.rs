use cgmath::{
    vec2, vec3, vec4, InnerSpace, Matrix, Matrix3, Matrix4, MetricSpace, Point3, Rad, Vector2,
    Vector3, Vector4,
};

const SPEED: f32 = 0.5;

pub struct Camera {
    pos: Vector3<f32>,
    target: Point3<f32>,
    last: Vector2<f32>,
}

impl Camera {
    pub fn new(radius: f32, target: Point3<f32>) -> Self {
        Self {
            pos: vec3(radius, 0., 0.),
            target,
            last: vec2(0., 0.),
        }
    }

    fn convert_to_cartesian(sc: Vector3<f32>) -> Point3<f32> {
        Point3::new(
            sc.x * sc.z.cos() * sc.y.cos(),
            sc.x * sc.z.sin(),
            sc.x * sc.z.cos() * sc.y.sin(),
        )
    }

    pub fn get_view(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(
            Self::convert_to_cartesian(self.pos),
            self.target,
            vec3(0., 1., 0.),
        )
    }

    pub fn set_last(&mut self, x: f32, y: f32) {
        self.last = vec2(x, y);
    }

    pub fn pan(&mut self, x: f32, y: f32, delta_time: f32) {
        let dx = (self.last.x - x) * SPEED;
        let dy = (self.last.y - y) * SPEED;
        if dx == 0f32 || dy == 0f32 {
            return;
        }

        self.pos.y -= dx * delta_time;
        self.pos.z = (self.pos.z - dy * delta_time).clamp(-1.5, 1.5);

        self.last.x = x;
        self.last.y = y;
    }
}
