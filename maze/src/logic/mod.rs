use anyhow::Result;
use cgmath::{vec3, Point3, Vector3};
use std::{fmt::Display, ops::Index, slice::Iter, str::FromStr};

#[derive(PartialEq, Eq)]
pub enum MazeEntry {
    Empty,
    Wall,
    Start,
    End,
}

impl From<&MazeEntry> for char {
    fn from(entry: &MazeEntry) -> Self {
        match entry {
            MazeEntry::Empty => '_',
            MazeEntry::Wall => '|',
            MazeEntry::Start => 'S',
            MazeEntry::End => 'E',
        }
    }
}

impl FromStr for MazeEntry {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "_" => Ok(MazeEntry::Empty),
            "|" => Ok(MazeEntry::Wall),
            "S" => Ok(MazeEntry::Start),
            "E" => Ok(MazeEntry::End),
            _ => anyhow::bail!("Invalid Str"),
        }
    }
}

pub struct Maze {
    maze: Vec<Vec<MazeEntry>>,
    player: MazeIndex,
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
        let maze: Vec<Vec<MazeEntry>> = s
            .lines()
            .enumerate()
            .map(|(ri, row)| {
                row.split(" ")
                    .enumerate()
                    .map(|(ci, entry)| {
                        let entry = entry.parse()?;
                        match entry {
                            MazeEntry::Start => {
                                start = MazeIndex::new(ci as isize, ri as isize);
                                has_start = true
                            }
                            MazeEntry::End => has_end = true,
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

        return Ok(Self::new(maze, start));
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

impl From<Point3<f32>> for MazeIndex {
    fn from(p: Point3<f32>) -> Self {
        let row_f = p.z;
        // 0.5 is the difference but I gave it an little extra padding of .01
        let row_f = if row_f - row_f.floor() > 0.49 {
            row_f.ceil() as isize
        } else {
            row_f.floor() as isize
        };
        let col_f = p.x;
        let col_f = if col_f - col_f.floor() > 0.49 {
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
    fn new(maze: Vec<Vec<MazeEntry>>, start: MazeIndex) -> Self {
        Self {
            maze,
            player: start,
        }
    }

    pub fn get_player_loc(&self) -> Vector3<f32> {
        self.player.into()
    }

    pub fn iter(&self) -> Iter<Vec<MazeEntry>> {
        self.maze.iter()
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
