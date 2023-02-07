use cgmath::{vec4, Vector4};
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

#[derive(Clone, Debug)]
pub struct Block {
    colors: [Colors; 6],
}
// FORWARD, BACKWARD, LEFT, RIGHT, UP, DOWN
impl Block {
    fn new(colors: [Colors; 6]) -> Self {
        Self { colors }
    }
    pub fn get_colors(&self) -> Vec<Vector4<f32>> {
        self.colors.iter().map(|x| x.into()).collect()
    }
}

#[derive(Debug)]
pub struct RubiksCube {
    blocks: [Block; 27],
}

impl RubiksCube {
    pub fn iter(&self) -> Iter<Block> {
        return self.blocks.iter();
    }

    pub fn new() -> Self {
        use Colors::*;
        Self {
            blocks: [
                Block::new([WHITE, EMPTY, BLUE, EMPTY, EMPTY, YELLOW]),
                Block::new([WHITE, EMPTY, BLUE, EMPTY, EMPTY, EMPTY]),
                Block::new([WHITE, EMPTY, BLUE, EMPTY, RED, EMPTY]),
                //
                Block::new([WHITE, EMPTY, EMPTY, EMPTY, EMPTY, YELLOW]),
                Block::new([WHITE, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY]),
                Block::new([WHITE, EMPTY, EMPTY, EMPTY, RED, EMPTY]),
                //
                Block::new([WHITE, EMPTY, EMPTY, GREEN, EMPTY, YELLOW]),
                Block::new([WHITE, EMPTY, EMPTY, GREEN, EMPTY, EMPTY]),
                Block::new([WHITE, EMPTY, EMPTY, GREEN, RED, EMPTY]),
                //
                Block::new([EMPTY, EMPTY, BLUE, EMPTY, EMPTY, YELLOW]),
                Block::new([EMPTY, EMPTY, BLUE, EMPTY, EMPTY, EMPTY]),
                Block::new([EMPTY, EMPTY, BLUE, EMPTY, RED, EMPTY]),
                //
                Block::new([EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, YELLOW]),
                Block::new([EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY]),
                Block::new([EMPTY, EMPTY, EMPTY, EMPTY, RED, EMPTY]),
                //
                Block::new([EMPTY, EMPTY, EMPTY, GREEN, EMPTY, YELLOW]),
                Block::new([EMPTY, EMPTY, EMPTY, GREEN, EMPTY, EMPTY]),
                Block::new([EMPTY, EMPTY, EMPTY, GREEN, RED, EMPTY]),
                //
                Block::new([EMPTY, GREY, BLUE, EMPTY, EMPTY, YELLOW]),
                Block::new([EMPTY, GREY, BLUE, EMPTY, EMPTY, EMPTY]),
                Block::new([EMPTY, GREY, BLUE, EMPTY, RED, EMPTY]),
                //
                Block::new([EMPTY, GREY, EMPTY, EMPTY, EMPTY, YELLOW]),
                Block::new([EMPTY, GREY, EMPTY, EMPTY, EMPTY, EMPTY]),
                Block::new([EMPTY, GREY, EMPTY, EMPTY, RED, EMPTY]),
                //
                Block::new([EMPTY, GREY, EMPTY, GREEN, EMPTY, YELLOW]),
                Block::new([EMPTY, GREY, EMPTY, GREEN, EMPTY, EMPTY]),
                Block::new([EMPTY, GREY, EMPTY, GREEN, RED, EMPTY]),
            ],
        }
    }

    pub fn rotate(&mut self, mut start: usize, is_horizontal: bool, is_clockwise: bool) {
        if is_horizontal {
            start = start % 3;
        } else {
            start = start / 9;
        }

        let step = if is_horizontal { 2 } else { 1 };

        let mat: Vec<_> = self
            .blocks
            .iter()
            .skip(start)
            .step_by(step)
            .take(9)
            .collect();
        println!("mat: {:?}", mat);

        let center = mat[4];
        let mut mat = vec![
            mat[0], mat[1], mat[2], mat[5], mat[8], mat[7], mat[6], mat[3],
        ];
        println!("mat ordered: {:?}", mat);
        if is_clockwise {
            mat.rotate_right(2);
        } else {
            mat.rotate_left(2);
        }
        mat.push(center);
        println!("mat rotated: {:?}", mat);
        let mat = vec![
            mat[0], mat[1], mat[2], mat[7], mat[8], mat[3], mat[6], mat[5], mat[4],
        ];
        let mat = mat.into_iter().cloned().collect::<Vec<_>>();

        for (old_block, new_block) in self
            .blocks
            .iter_mut()
            .skip(start)
            .step_by(step)
            .take(9)
            .zip(mat)
        {
            *old_block = new_block;
        }
    }
}
