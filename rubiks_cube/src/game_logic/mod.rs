mod colors;
use anyhow::{bail, Context, Result};
use cgmath::{vec3, Deg, Matrix4, SquareMatrix};
use colors::Colors;
use std::{
    collections::VecDeque,
    convert::{TryFrom, TryInto},
    slice::Iter,
};

struct ShadowPlane {
    //       face  ,  y    , x
    pub plane: [(usize, [(usize, usize); 3]); 4],
}

impl TryFrom<usize> for ShadowPlane {
    type Error = anyhow::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self {
                plane: [
                    (3, [(2, 0), (2, 1), (2, 2)]),
                    (2, [(2, 0), (2, 1), (2, 2)]),
                    (1, [(2, 0), (2, 1), (2, 2)]),
                    (4, [(2, 0), (2, 1), (2, 2)]),
                ],
            }),
            1 => Ok(Self {
                plane: [
                    (0, [(0, 2), (0, 1), (0, 0)]),
                    (2, [(0, 0), (1, 0), (2, 0)]),
                    (5, [(0, 0), (0, 1), (0, 2)]),
                    (4, [(0, 0), (1, 0), (2, 0)]),
                ],
            }),
            2 => Ok(Self {
                plane: [
                    (0, [(0, 0), (1, 0), (2, 0)]),
                    (3, [(2, 0), (1, 0), (0, 0)]),
                    (5, [(2, 0), (1, 0), (0, 0)]),
                    (1, [(0, 0), (1, 0), (2, 0)]),
                ],
            }),
            3 => Ok(Self {
                plane: [
                    (0, [(2, 0), (2, 1), (2, 2)]),
                    (4, [(0, 2), (1, 2), (2, 2)]),
                    (5, [(2, 2), (2, 1), (2, 0)]),
                    (2, [(0, 2), (1, 2), (2, 2)]),
                ],
            }),
            4 => Ok(Self {
                plane: [
                    (0, [(0, 2), (1, 2), (2, 2)]),
                    (1, [(0, 2), (1, 2), (2, 2)]),
                    (5, [(0, 2), (1, 2), (2, 2)]),
                    (3, [(0, 2), (1, 2), (2, 2)]),
                ],
            }),
            5 => Ok(Self {
                plane: [
                    (3, [(0, 0), (0, 1), (0, 2)]),
                    (2, [(0, 0), (0, 1), (0, 2)]),
                    (1, [(0, 0), (0, 1), (0, 2)]),
                    (4, [(0, 0), (0, 1), (0, 2)]),
                ],
            }),
            6 => Ok(Self {
                plane: [
                    (0, [(1, 0), (1, 1), (1, 2)]),
                    (4, [(0, 1), (1, 1), (2, 1)]),
                    (5, [(1, 0), (1, 1), (1, 2)]),
                    (2, [(0, 1), (1, 1), (2, 1)]),
                ],
            }),
            7 => Ok(Self {
                plane: [
                    (3, [(1, 0), (1, 1), (1, 2)]),
                    (4, [(1, 0), (1, 1), (1, 2)]),
                    (1, [(1, 0), (1, 1), (1, 2)]),
                    (2, [(1, 0), (1, 1), (1, 2)]),
                ],
            }),
            8 => Ok(Self {
                plane: [
                    (3, [(0, 1), (1, 1), (2, 1)]),
                    (0, [(0, 1), (1, 1), (2, 1)]),
                    (1, [(0, 1), (1, 1), (2, 1)]),
                    (5, [(0, 1), (1, 1), (2, 1)]),
                ],
            }),
            _ => bail!("Only values between 0-8 are valid"),
        }
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
                    Matrix4::from_angle_x(Deg(90.)),
                    [[RED, RED, RED], [RED, RED, RED], [RED, RED, RED]],
                    Box::new(|x, y| Matrix4::from_translation(vec3(x as f32, 3., y as f32))),
                ),
                Face::new(
                    Matrix4::identity(),
                    [[GREY, GREY, GREY], [GREY, GREY, GREY], [GREY, GREY, GREY]],
                    Box::new(|x, y| Matrix4::from_translation(vec3(x as f32, y as f32, -1.))),
                ),
                Face::new(
                    Matrix4::from_angle_y(Deg(90.)),
                    [[BLUE, BLUE, BLUE], [BLUE, BLUE, BLUE], [BLUE, BLUE, BLUE]],
                    Box::new(|x, y| Matrix4::from_translation(vec3(-1., y as f32, x as f32))),
                ),
                Face::new(
                    Matrix4::identity(),
                    [
                        [WHITE, WHITE, WHITE],
                        [WHITE, WHITE, WHITE],
                        [WHITE, WHITE, WHITE],
                    ],
                    Box::new(|x, y| Matrix4::from_translation(vec3(x as f32, y as f32, 2.))),
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
            ],
        }
    }

    pub fn rotate(&mut self, face: usize, is_clockwise: bool) -> Result<()> {
        let shadow_plane_cords: ShadowPlane = face.try_into()?;

        let mut shadow_plane = shadow_plane_cords
            .plane
            .iter()
            .flat_map(|(face, cords)| cords.map(|(y, x)| self.blocks[*face].faces[y][x].clone()))
            .collect::<VecDeque<_>>();

        if is_clockwise {
            shadow_plane.rotate_right(3);
        } else {
            shadow_plane.rotate_left(3);
        }

        for (face, cords) in shadow_plane_cords.plane.iter() {
            for (y, x) in cords.iter().cloned() {
                self.blocks[*face].faces[y][x] = shadow_plane
                    .pop_front()
                    .context("Error rotating shadow plane")?;
            }
        }

        if face < 6 {
            // the current face
            let mut new_face: [[Colors; 3]; 3] = self.blocks[face].faces.clone();
            let mut iter_holder_a;
            let mut iter_holder_b;
            let iter: &mut dyn Iterator<Item = &[Colors; 3]> = if is_clockwise {
                iter_holder_a = self.blocks[face].iter().rev();
                &mut iter_holder_a
            } else {
                iter_holder_b = self.blocks[face].iter();
                &mut iter_holder_b
            };

            for (y, row) in iter.enumerate() {
                for (x, color) in row.iter().enumerate() {
                    new_face[x][y] = color.clone()
                }
            }
            self.blocks[face].faces = new_face;
        }
        // do the shadow face inner planes only have shadow faces
        return Ok(());
    }
}
