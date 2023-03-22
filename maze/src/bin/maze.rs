use anyhow::Result;
use std::fs::read_to_string;

use maze::logic::Maze;

fn main() {
    let mut maze: Maze = read_to_string("./maze.txt")
        .expect("Failed to find file maze.txt")
        .parse()
        .expect("Error parsing maze");
}
