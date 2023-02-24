mod colors;
use anyhow::{bail, Context, Result};
use cgmath::{vec3, Deg, Matrix4, Rad, SquareMatrix, Vector3};
use colors::Colors;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque,
    convert::{TryFrom, TryInto},
    f32::consts::PI,
    slice::Iter,
};

pub struct ShadowPlane {
    //       face  ,  y    , x
    pub plane: [(usize, [(usize, usize); 3], usize); 4],
}

impl TryFrom<usize> for ShadowPlane {
    type Error = anyhow::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self {
                plane: [
                    (3, [(2, 2), (2, 1), (2, 0)], 9),
                    (2, [(2, 2), (2, 1), (2, 0)], 9),
                    (1, [(2, 0), (2, 1), (2, 2)], 9),
                    (4, [(2, 0), (2, 1), (2, 2)], 9),
                ],
            }),
            1 => Ok(Self {
                plane: [
                    (0, [(0, 2), (0, 1), (0, 0)], 9),
                    (2, [(2, 0), (1, 0), (0, 0)], 9),
                    (5, [(0, 0), (0, 1), (0, 2)], 9),
                    (4, [(0, 0), (1, 0), (2, 0)], 9),
                ],
            }),
            2 => Ok(Self {
                plane: [
                    (0, [(0, 0), (1, 0), (2, 0)], 9),
                    (3, [(2, 0), (1, 0), (0, 0)], 9),
                    (5, [(2, 0), (1, 0), (0, 0)], 9),
                    (1, [(0, 0), (1, 0), (2, 0)], 9),
                ],
            }),
            3 => Ok(Self {
                plane: [
                    (0, [(2, 0), (2, 1), (2, 2)], 9),
                    (4, [(2, 2), (1, 2), (0, 2)], 9),
                    (5, [(2, 2), (2, 1), (2, 0)], 9),
                    (2, [(0, 2), (1, 2), (2, 2)], 9),
                ],
            }),
            4 => Ok(Self {
                plane: [
                    (0, [(0, 2), (1, 2), (2, 2)], 9),
                    (1, [(0, 2), (1, 2), (2, 2)], 9),
                    (5, [(2, 2), (1, 2), (0, 2)], 9),
                    (3, [(2, 2), (1, 2), (0, 2)], 9),
                ],
            }),
            5 => Ok(Self {
                plane: [
                    (3, [(0, 0), (0, 1), (0, 2)], 9),
                    (4, [(0, 2), (0, 1), (0, 0)], 9),
                    (1, [(0, 2), (0, 1), (0, 0)], 9),
                    (2, [(0, 0), (0, 1), (0, 2)], 9),
                ],
            }),
            6 => Ok(Self {
                plane: [
                    (0, [(1, 0), (1, 1), (1, 2)], 10),
                    (2, [(0, 1), (1, 1), (2, 1)], 12),
                    (5, [(1, 2), (1, 1), (1, 0)], 9),
                    (4, [(2, 1), (1, 1), (0, 1)], 11),
                ],
            }),
            7 => Ok(Self {
                plane: [
                    (3, [(1, 0), (1, 1), (1, 2)], 10),
                    (4, [(1, 0), (1, 1), (1, 2)], 10),
                    (1, [(1, 2), (1, 1), (1, 0)], 10),
                    (2, [(1, 2), (1, 1), (1, 0)], 10),
                ],
            }),
            8 => Ok(Self {
                plane: [
                    (3, [(0, 1), (1, 1), (2, 1)], 11),
                    (5, [(0, 1), (1, 1), (2, 1)], 11),
                    (1, [(0, 1), (1, 1), (2, 1)], 12),
                    (0, [(0, 1), (1, 1), (2, 1)], 12),
                ],
            }),
            _ => bail!("Only values between 0-8 are valid"),
        }
    }
}

#[derive(Serialize)]
pub struct Face {
    faces: [[Colors; 3]; 3],
    #[serde(skip_serializing)]
    rotation: Matrix4<f32>,
    #[serde(skip_serializing)]
    convert_cord: Box<dyn Fn(f32, f32) -> Vector3<f32>>,
}

impl Face {
    fn new(
        rotation: Matrix4<f32>,
        faces: [[Colors; 3]; 3],
        convert_cord: Box<dyn Fn(f32, f32) -> Vector3<f32>>,
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
        return Matrix4::from_translation((self.convert_cord)(x, y));
    }
}

#[derive(Serialize)]
pub struct RubiksCube {
    blocks: [Face; 6],
}

impl RubiksCube {
    pub fn get_rotate_matrix(
        &self,
        rotating_face: usize,
        face: usize,
        x: f32,
        y: f32,
        p: f64,
        is_clockwise: bool,
    ) -> Result<Matrix4<f32>> {
        if rotating_face >= 9 || face >= 6 {
            bail!("Unsupported face")
        }
        let center = if rotating_face >= 6 {
            vec3(1., 1., 1.)
        } else {
            (self.blocks[rotating_face].convert_cord)(1., 1.)
        };

        let v = (self.blocks[face].convert_cord)(x, y) - center;
        return Ok(Matrix4::from_translation(-v)
            * self
                .get_face_rotate(rotating_face, p, is_clockwise)
                .unwrap()
            * Matrix4::from_translation(v));
    }
    fn get_face_rotate(&self, face: usize, mut p: f64, is_clockwise: bool) -> Result<Matrix4<f32>> {
        p = match face {
            5 | 7 | 4 | 3 => -p,
            _ => p,
        };
        if !is_clockwise {
            p = -p;
        }
        match face {
            3 | 1 | 6 => Ok(Matrix4::from_angle_z(Rad(p as f32 * PI / 2.))),
            0 | 5 | 7 => Ok(Matrix4::from_angle_y(Rad(-p as f32 * PI / 2.))),
            2 | 4 | 8 => Ok(Matrix4::from_angle_x(Rad(p as f32 * PI / 2.))),
            _ => bail!("Unsupported face"),
        }
    }

    pub fn iter(&self) -> Iter<Face> {
        return self.blocks.iter();
    }

    pub fn save(&self, path: &str) -> Result<()> {
        let c = serde_json::to_string(self)?;
        std::fs::write(path, c)?;
        Ok(())
    }

    pub fn new_from_save(p: &str) -> Result<Self> {
        let contents =
            std::fs::read_to_string("./rubiks_cube_save.txt").context("failed to read")?;
        #[derive(Deserialize, Clone)]
        struct _FC {
            pub faces: [[Colors; 3]; 3],
        }
        #[derive(Deserialize)]
        struct _RC {
            pub blocks: [_FC; 6],
        }
        let rc: _RC = serde_json::from_str(&contents).context("failed to serialize")?;
        let b = &rc.blocks;

        Ok(Self {
            blocks: [
                Face::new(
                    Matrix4::from_angle_x(Deg(90.)),
                    b[0].faces.clone(),
                    Box::new(|x, y| (vec3(x as f32, 3., y as f32))),
                ),
                Face::new(
                    Matrix4::identity(),
                    b[1].faces.clone(),
                    Box::new(|x, y| (vec3(x as f32, y as f32, -1.))),
                ),
                Face::new(
                    Matrix4::from_angle_y(Deg(90.)),
                    b[2].faces.clone(),
                    Box::new(|x, y| (vec3(-1., y as f32, x as f32))),
                ),
                Face::new(
                    Matrix4::identity(),
                    b[3].faces.clone(),
                    Box::new(|x, y| (vec3(x as f32, y as f32, 2.))),
                ),
                Face::new(
                    Matrix4::from_angle_y(Deg(90.)),
                    b[4].faces.clone(),
                    Box::new(|x, y| (vec3(2., y as f32, x as f32))),
                ),
                Face::new(
                    Matrix4::from_angle_x(Deg(90.)),
                    b[5].faces.clone(),
                    Box::new(|x, y| (vec3(x as f32, 0., y as f32))),
                ),
            ],
        })
    }

    pub fn new() -> Self {
        use Colors::*;
        let mut cube = Self {
            blocks: [
                Face::new(
                    Matrix4::from_angle_x(Deg(90.)),
                    [[RED, RED, RED], [RED, RED, RED], [RED, RED, RED]],
                    Box::new(|x, y| (vec3(x as f32, 3., y as f32))),
                ),
                Face::new(
                    Matrix4::identity(),
                    [[GREY, GREY, GREY], [GREY, GREY, GREY], [GREY, GREY, GREY]],
                    Box::new(|x, y| (vec3(x as f32, y as f32, -1.))),
                ),
                Face::new(
                    Matrix4::from_angle_y(Deg(90.)),
                    [[BLUE, BLUE, BLUE], [BLUE, BLUE, BLUE], [BLUE, BLUE, BLUE]],
                    Box::new(|x, y| (vec3(-1., y as f32, x as f32))),
                ),
                Face::new(
                    Matrix4::identity(),
                    [
                        [WHITE, WHITE, WHITE],
                        [WHITE, WHITE, WHITE],
                        [WHITE, WHITE, WHITE],
                    ],
                    Box::new(|x, y| (vec3(x as f32, y as f32, 2.))),
                ),
                Face::new(
                    Matrix4::from_angle_y(Deg(90.)),
                    [
                        [GREEN, GREEN, GREEN],
                        [GREEN, GREEN, GREEN],
                        [GREEN, GREEN, GREEN],
                    ],
                    Box::new(|x, y| (vec3(2., y as f32, x as f32))),
                ),
                Face::new(
                    Matrix4::from_angle_x(Deg(90.)),
                    [
                        [YELLOW, YELLOW, YELLOW],
                        [YELLOW, YELLOW, YELLOW],
                        [YELLOW, YELLOW, YELLOW],
                    ],
                    Box::new(|x, y| (vec3(x as f32, 0., y as f32))),
                ),
            ],
        };
        cube.shuffle();
        return cube;
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();

        for _ in 0..rng.gen_range(50..=100) {
            self.rotate(rng.gen_range(0..=8usize), rng.gen_bool(0.5))
                .expect("rotate should always be between 0 and 8");
        }
    }

    pub fn rotate(&mut self, face: usize, mut is_clockwise: bool) -> Result<()> {
        let shadow_plane_cords: ShadowPlane = face.try_into()?;

        let mut shadow_plane = shadow_plane_cords
            .plane
            .iter()
            .flat_map(|(face, cords, _)| cords.map(|(y, x)| self.blocks[*face].faces[y][x].clone()))
            .collect::<VecDeque<_>>();

        if is_clockwise {
            shadow_plane.rotate_right(3);
        } else {
            shadow_plane.rotate_left(3);
        }

        for (face, cords, _) in shadow_plane_cords.plane.iter() {
            for (y, x) in cords.iter().cloned() {
                self.blocks[*face].faces[y][x] = shadow_plane
                    .pop_front()
                    .context("Error rotating shadow plane")?;
            }
        }

        if face < 6 {
            // the current face
            is_clockwise = match face {
                2 | 5 | 3 => !is_clockwise,
                _ => is_clockwise,
            };
            let face_cords = [
                (0, 0),
                (0, 1),
                (0, 2),
                (1, 2),
                (2, 2),
                (2, 1),
                (2, 0),
                (1usize, 0usize),
            ];
            let mut new_face = face_cords
                .iter()
                .map(|&(y, x)| self.blocks[face].faces[y][x].clone())
                .collect::<Vec<_>>();

            if is_clockwise {
                new_face.rotate_right(2);
            } else {
                new_face.rotate_left(2);
            };

            for ((y, x), color) in face_cords.into_iter().zip(new_face.into_iter()) {
                self.blocks[face].faces[y][x] = color;
            }
        }
        return Ok(());
    }
}
