use anyhow::Result;
use cgmath::{vec3, EuclideanSpace, Matrix4, Point3, Vector3};
use learn_opengl::lights::point_lights::{PointLight, PointLightBuilder};
use std::{fmt::Display, ops::Index, slice::Iter, str::FromStr};

#[derive(PartialEq, Eq, Debug)]
pub enum MazeEntry {
    Empty(bool),
    Wall,
    Start,
    End,
}

impl From<&MazeEntry> for char {
    fn from(entry: &MazeEntry) -> Self {
        match entry {
            MazeEntry::Empty(false) => '_',
            MazeEntry::Wall => '|',
            MazeEntry::Empty(true) => 'L',
            MazeEntry::Start => 'S',
            MazeEntry::End => 'E',
        }
    }
}

impl FromStr for MazeEntry {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "_" => Ok(MazeEntry::Empty(false)),
            "|" => Ok(MazeEntry::Wall),
            "L" => Ok(MazeEntry::Empty(true)),
            "S" => Ok(MazeEntry::Start),
            "E" => Ok(MazeEntry::End),
            _ => anyhow::bail!("Invalid Str"),
        }
    }
}

#[derive(Debug)]
pub struct Maze {
    maze: Vec<Vec<MazeEntry>>,
    player: MazeIndex,
    lights: Vec<PointLight>,
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (ri, row) in self.maze.iter().enumerate() {
            for (ci, entry) in row.iter().enumerate() {
                let c: char = if MazeIndex::new(ri as isize, ci as isize) == self.player {
                    'P'
                } else {
                    entry.into()
                };
                if ci == 0 {
                    write!(f, "{}", c)?;
                } else {
                    write!(f, " {}", c)?;
                }
            }
            writeln!(f, "")?;
        }
        return Ok(());
    }
}

impl FromStr for Maze {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = MazeIndex::new(0, 0);
        let mut has_start = false;
        let mut has_end = false;
        let mut lights = vec![];
        let mut start_light = 2;
        let maze: Vec<Vec<MazeEntry>> = s
            .lines()
            .enumerate()
            .map(|(ri, row)| {
                row.split(" ")
                    .enumerate()
                    .map(|(ci, entry)| {
                        let entry = entry.parse()?;
                        use MazeEntry::*;
                        match entry {
                            Start => {
                                start = MazeIndex::new(ri as isize, ci as isize);
                                has_start = true;
                                lights.push(
                                    PointLightBuilder::default()
                                        .pos(start.into())
                                        .color(vec3(0.0, 0.0, 1.0))
                                        .depth_map(start_light)
                                        .build()?,
                                );
                                start_light += 1;
                            }
                            End => {
                                has_end = true;
                                lights.push(
                                    PointLightBuilder::default()
                                        .pos(MazeIndex::new(ri as isize, ci as isize).into())
                                        .color(vec3(0.0, 1.0, 1.0))
                                        .depth_map(start_light)
                                        .build()?,
                                );
                                start_light += 1;
                            }
                            Empty(true) => {
                                lights.push(
                                    PointLightBuilder::default()
                                        .pos(MazeIndex::new(ri as isize, ci as isize).into())
                                        .depth_map(start_light)
                                        .build()?,
                                );
                                start_light += 1;
                            }
                            _ => {}
                        };
                        return Ok(entry);
                    })
                    .collect::<Result<_>>()
            })
            .collect::<Result<_>>()?;

        if !has_start {
            anyhow::bail!("Invalid No Start point defined")
        }
        if !has_end {
            anyhow::bail!("Invalid No End point defined")
        }

        return Ok(Self::new(maze, start, lights));
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct MazeIndex {
    row: isize,
    col: isize,
}

impl MazeIndex {
    pub fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }
}

impl From<MazeIndex> for Vector3<f32> {
    fn from(mi: MazeIndex) -> Self {
        vec3(mi.col as f32, 1.0, mi.row as f32)
    }
}

impl From<Vector3<f32>> for MazeIndex {
    fn from(v: Vector3<f32>) -> Self {
        Point3::from_vec(v).into()
    }
}

impl From<Point3<f32>> for MazeIndex {
    fn from(p: Point3<f32>) -> Self {
        let row_f = p.z;
        let col_f = p.x;
        let row_f = if row_f - row_f.floor() > 0.5 {
            row_f.ceil() as isize
        } else {
            row_f.floor() as isize
        };
        let col_f = if col_f - col_f.floor() > 0.5 {
            col_f.ceil() as isize
        } else {
            col_f.floor() as isize
        };
        Self::new(row_f, col_f)
    }
}

impl Index<MazeIndex> for Maze {
    type Output = MazeEntry;

    fn index(&self, index: MazeIndex) -> &Self::Output {
        &self.maze[index.row as usize][index.col as usize]
    }
}

impl Maze {
    fn new(maze: Vec<Vec<MazeEntry>>, start: MazeIndex, lights: Vec<PointLight>) -> Self {
        Self {
            maze,
            player: start,
            lights,
        }
    }

    pub fn get_player_loc(&self) -> Vector3<f32> {
        self.player.into()
    }

    pub fn iter(&self) -> Iter<Vec<MazeEntry>> {
        self.maze.iter()
    }

    pub fn get_lights(&self) -> &Vec<PointLight> {
        &self.lights
    }

    pub fn move_player_to(&mut self, new_loc: MazeIndex) {
        self.player = new_loc;
    }

    pub fn move_player(&mut self, dx: isize, dy: isize) -> Result<()> {
        if dx.abs() > 1 || dy.abs() > 1 {
            anyhow::bail!("Can't move by more than one")
        }

        let new_cords = MazeIndex::new(self.player.row + dy, self.player.col + dx);

        println!("new cords: {:?}, old cords: {:?}", new_cords, self.player);

        return match self[new_cords] {
            MazeEntry::Wall => anyhow::bail!("Can't move into a wall"),
            _ => {
                self.player = new_cords;
                Ok(())
            }
        };
    }
}
