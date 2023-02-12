use cgmath::{vec3, vec4, Deg, Matrix4, Rotation, SquareMatrix, Vector4};
use std::slice::Iter;
#[derive(Clone, Debug)]
pub enum Colors {
    RED,
    BLUE,
    GREEN,
    YELLOW,
    WHITE,
    GREY,
    EMPTY,
}

impl Colors {
    fn convert_to_rbga(&self) -> Vector4<f32> {
        match self {
            Colors::RED => vec4(1., 0., 0., 1.),
            Colors::BLUE => vec4(0., 0., 1., 1.),
            Colors::GREEN => vec4(0., 1., 0., 1.),
            Colors::YELLOW => vec4(1., 1., 0., 1.),
            Colors::WHITE => vec4(1., 1., 1., 1.),
            Colors::GREY => vec4(0.75, 0.75, 0.75, 1.),
            Colors::EMPTY => vec4(0., 0., 0., 1.),
        }
    }
}

impl From<Colors> for Vector4<f32> {
    fn from(color: Colors) -> Self {
        color.convert_to_rbga()
    }
}

impl From<&Colors> for Vector4<f32> {
    fn from(color: &Colors) -> Self {
        color.convert_to_rbga()
    }
}

pub struct Face {
    faces: [[Colors; 3]; 3],
    rotation: Matrix4<f32>,
    convert_cord: Box<dyn Fn(f32, f32) -> Matrix4<f32>>,
}

impl Face {
    fn new(
        rotation: Matrix4<f32>,
        faces: [[Colors; 3]; 3],
        convert_cord: Box<dyn Fn(f32, f32) -> Matrix4<f32>>,
    ) -> Self {
        Self {
            faces,
            rotation,
            convert_cord,
        }
    }
    pub fn iter(&self) -> Iter<[Colors; 3]> {
        self.faces.iter()
    }
    pub fn get_rotation(&self) -> Matrix4<f32> {
        self.rotation
    }
    pub fn convert_cords(&self, x: f32, y: f32) -> Matrix4<f32> {
        return (self.convert_cord)(x, y);
    }
}

pub struct RubiksCube {
    blocks: [Face; 6],
}

impl RubiksCube {
    pub fn iter(&self) -> Iter<Face> {
        return self.blocks.iter();
    }

    pub fn new() -> Self {
        use Colors::*;
        Self {
            blocks: [
                Face::new(
                    Matrix4::identity(),
                    [
                        [WHITE, WHITE, WHITE],
                        [WHITE, WHITE, WHITE],
                        [WHITE, WHITE, WHITE],
                    ],
                    Box::new(|x, y| Matrix4::from_translation(vec3(x as f32, y as f32, -1.))),
                ),
                Face::new(
                    Matrix4::identity(),
                    [[GREY, GREY, GREY], [GREY, GREY, GREY], [GREY, GREY, GREY]],
                    Box::new(|x, y| Matrix4::from_translation(vec3(x as f32, y as f32, 2.))),
                ),
                Face::new(
                    Matrix4::from_angle_y(Deg(90.)),
                    [[BLUE, BLUE, BLUE], [BLUE, BLUE, BLUE], [BLUE, BLUE, BLUE]],
                    Box::new(|x, y| Matrix4::from_translation(vec3(-1., y as f32, x as f32))),
                ),
                Face::new(
                    Matrix4::from_angle_y(Deg(90.)),
                    [
                        [GREEN, GREEN, GREEN],
                        [GREEN, GREEN, GREEN],
                        [GREEN, GREEN, GREEN],
                    ],
                    Box::new(|x, y| Matrix4::from_translation(vec3(2., y as f32, x as f32))),
                ),
                Face::new(
                    Matrix4::from_angle_x(Deg(90.)),
                    [
                        [YELLOW, YELLOW, YELLOW],
                        [YELLOW, YELLOW, YELLOW],
                        [YELLOW, YELLOW, YELLOW],
                    ],
                    Box::new(|x, y| Matrix4::from_translation(vec3(x as f32, 0., y as f32))),
                ),
                Face::new(
                    Matrix4::from_angle_x(Deg(90.)),
                    [[RED, RED, RED], [RED, RED, RED], [RED, RED, RED]],
                    Box::new(|x, y| Matrix4::from_translation(vec3(x as f32, 3., y as f32))),
                ),
            ],
        }
    }

    pub fn rotate(&mut self) {
        todo!("")
    }
}
