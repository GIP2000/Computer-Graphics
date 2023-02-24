use cgmath::{vec4, Vector4};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
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
