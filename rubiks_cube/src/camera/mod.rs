use cgmath::{vec2, vec3, Matrix4, Point3, Vector2, Vector3};

const SPEED: f32 = 0.5;

pub struct Camera {
    pos: Vector3<f32>,
    target: Point3<f32>,
    last_mouse: Vector2<f32>,
}

impl Camera {
    pub fn new(pos: Point3<f32>, target: Point3<f32>) -> Self {
        Self {
            pos: Self::convert_to_spherical(pos),
            target,
            last_mouse: vec2(0., 0.),
        }
    }

    fn convert_to_spherical(ct: Point3<f32>) -> Vector3<f32> {
        let r = (ct.x.powf(2.) + ct.y.powf(2.) + ct.z.powf(2.)).sqrt();
        let phi = (ct.z / ct.x).atan2(ct.x);
        let theta = (ct.y / r).acos();
        vec3(r, phi, theta)
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
        self.last_mouse = vec2(x, y);
    }

    pub fn pan(&mut self, x: f32, y: f32, delta_time: f32) {
        let dx = (self.last_mouse.x - x) * SPEED;
        let dy = (self.last_mouse.y - y) * SPEED;
        if dx == 0f32 || dy == 0f32 {
            return;
        }

        self.pos.y -= dx * delta_time;
        self.pos.z = (self.pos.z - dy * delta_time).clamp(-1.5, 1.5);

        self.last_mouse.x = x;
        self.last_mouse.y = y;
    }
}
