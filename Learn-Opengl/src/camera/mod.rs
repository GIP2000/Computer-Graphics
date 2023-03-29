use cgmath::{vec3, ElementWise, EuclideanSpace, InnerSpace, Matrix4, Point2, Point3, Vector3};

pub type CameraDirection = u8;
// bit packing
// 0000 0000
//   ^^ ^^^^
//   DU LRBF

pub trait CameraDirectionTrait {
    const FORWARD: u8;
    const BACKWARD: u8;
    const RIGHT: u8;
    const LEFT: u8;
    const UP: u8;
    const DOWN: u8;

    fn new() -> Self;
    fn is_forward(&self) -> f32;
    fn is_backward(&self) -> f32;
    fn is_right(&self) -> f32;
    fn is_left(&self) -> f32;
    fn is_up(&self) -> f32;
    fn is_down(&self) -> f32;

    fn toggle_forward(&mut self);
    fn toggle_backward(&mut self);
    fn toggle_right(&mut self);
    fn toggle_left(&mut self);
    fn toggle_up(&mut self);
    fn toggle_down(&mut self);
}

impl CameraDirectionTrait for CameraDirection {
    const FORWARD: u8 = 1;
    const BACKWARD: u8 = 2;
    const RIGHT: u8 = 4;
    const LEFT: u8 = 8;
    const UP: u8 = 16;
    const DOWN: u8 = 32;

    #[inline(always)]
    fn new() -> Self {
        0
    }

    #[inline(always)]
    fn toggle_forward(&mut self) {
        *self ^= Self::FORWARD;
    }
    #[inline(always)]
    fn toggle_backward(&mut self) {
        *self ^= Self::BACKWARD;
    }
    #[inline(always)]
    fn toggle_right(&mut self) {
        *self ^= Self::RIGHT;
    }
    #[inline(always)]
    fn toggle_left(&mut self) {
        *self ^= Self::LEFT;
    }

    #[inline(always)]
    fn toggle_down(&mut self) {
        *self ^= Self::DOWN;
    }

    #[inline(always)]
    fn toggle_up(&mut self) {
        *self ^= Self::UP;
    }

    #[inline(always)]
    fn is_forward(&self) -> f32 {
        (self & Self::FORWARD == Self::FORWARD) as u8 as f32
    }

    #[inline(always)]
    fn is_backward(&self) -> f32 {
        (self & Self::BACKWARD == Self::BACKWARD) as u8 as f32
    }

    #[inline(always)]
    fn is_right(&self) -> f32 {
        (self & Self::RIGHT == Self::RIGHT) as u8 as f32
    }

    #[inline(always)]
    fn is_left(&self) -> f32 {
        (self & Self::LEFT == Self::LEFT) as u8 as f32
    }

    #[inline(always)]
    fn is_up(&self) -> f32 {
        (self & Self::UP == Self::UP) as u8 as f32
    }

    #[inline(always)]
    fn is_down(&self) -> f32 {
        (self & Self::DOWN == Self::DOWN) as u8 as f32
    }
}

pub struct Camera {
    pos: Point3<f32>,
    true_up: Vector3<f32>,
    dir: Vector3<f32>,
    camera_right: Vector3<f32>,
    camera_up: Vector3<f32>,
    pub speed: Vector3<f32>,
    pub sensitivity: f32,
    last_cords: Option<Point2<f32>>,
    yaw: f32,
    pitch: f32,
}

impl Camera {
    fn calc_camera_up(dir: Vector3<f32>, right: Vector3<f32>) -> Vector3<f32> {
        dir.cross(right)
    }
    pub fn get_pos(&self) -> Vector3<f32> {
        self.pos.to_vec()
    }

    fn calc_camera_right(up: Vector3<f32>, dir: Vector3<f32>) -> Vector3<f32> {
        up.cross(dir).normalize()
    }
    fn calc_dir(yaw: f32, pitch: f32) -> Vector3<f32> {
        vec3(
            yaw.to_radians().cos() * pitch.to_radians().cos(),
            pitch.to_radians().sin(),
            yaw.to_radians().sin() * pitch.to_radians().cos(),
        )
        .normalize()
    }

    pub fn new(pos: Point3<f32>, yaw: f32, pitch: f32, speed: Vector3<f32>) -> Self {
        let true_up = vec3(0.0, 1.0, 0.0);
        let dir = Self::calc_dir(yaw, pitch);
        let camera_right = Self::calc_camera_right(true_up, dir);
        let camera_up = Self::calc_camera_up(dir, camera_right);
        Self {
            pos,
            true_up,
            dir,
            camera_right,
            camera_up,
            speed,
            sensitivity: 0.1,
            last_cords: None,
            yaw,
            pitch,
        }
    }

    pub fn move_point_pos(&mut self, x: f32, y: f32) {
        if let Some(last_cords) = self.last_cords {
            let x_offset = (x - last_cords.x) * self.sensitivity;
            let y_offset = (y - last_cords.y) * self.sensitivity;
            self.yaw += x_offset;
            self.pitch = (self.pitch + y_offset).min(89.).max(-89.);
            self.dir = vec3(
                self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
                self.pitch.to_radians().sin(),
                self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
            )
            .normalize();
        }
        self.last_cords = Some(Point2::new(x, y));
    }

    pub fn get_view(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(self.pos, self.pos - self.dir, self.camera_up)
    }

    pub fn set_x(&mut self, x: f32) {
        self.pos.x = x;
        self.recalc();
    }

    pub fn set_y(&mut self, y: f32) {
        self.pos.y = y;
        self.recalc();
    }

    pub fn set_z(&mut self, z: f32) {
        self.pos.z = z;
        self.recalc();
    }

    pub fn try_translate_camera(&mut self, dir: CameraDirection, delta_time: f32) -> Point3<f32> {
        let mut x =
            (dir.is_right() - dir.is_left()) * Self::calc_camera_right(self.camera_up, self.dir);
        x.y = 0.;
        let y = (dir.is_up() - dir.is_down()) * self.camera_up;
        let mut z = (dir.is_backward() - dir.is_forward()) * self.dir;
        z.y = 0.;
        return Point3::from_vec(
            self.pos.to_vec() + (x + y + z).mul_element_wise(self.speed) * delta_time,
        );
    }

    pub fn translate_camera(&mut self, dir: CameraDirection, delta_time: f32) {
        self.pos = self.try_translate_camera(dir, delta_time);
    }

    fn recalc(&mut self) {
        // self.dir = Self::calc_dir(self.pos.to_vec(), target);
        self.camera_right = Self::calc_camera_right(self.true_up, self.dir);
        self.camera_up = Self::calc_camera_up(self.dir, self.camera_right);
    }
}
