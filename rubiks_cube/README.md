# Rubicks Cube

## Installation

1. Install Rust
  - Install Rustup (Linux / Macos)
    - curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  - Windows
    - [Install Script Download](https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe)
  - run cargo --version to ensure rust is installed properly
2. clone repository
    - `git clone https://github.com/GIP2000/Learn-Opengl.git`
3. Install CMake
    -  [Installation Instructions](https://cmake.org/install/)
4. Install glfw
    - On Linux (Debian)
        - `sudo apt-get install libglfw3`
        - `sudo apt-get install libglfw3-dev`
    - On Mac
        `brew install glfw3` (might need to run `brew tap homebrew/versions` first)
    - On Windows

## Usage

In order to run the project run the command
`cargo run --bin rubiks_cube [Number of random rotations || name of save file]`

### Arguments
You can optionally pass in a number as a command line argument or a string
- If the argument is a number it is the number of rotations that occur randomly to mix up the Cube
- If the argumennt is a string the program will attempt to open and parse the file. If succesfull it will load the Rubicks Cube otherwise it will exit
- If no argument is passed it will display a non-randomized cube

### Controls
- Drag with the mouse to rotate the cube.
- Numbers on the center cube rotate the cube clockwise pressing shift rotates the cube in the opposite direction
- Pressing right click or the `s` key will show you the inner plane controls where the number and direction are on the center cube and once again shift and the number will rotate in the oppsite direction
- Pressing `w` will save the game to the file.

