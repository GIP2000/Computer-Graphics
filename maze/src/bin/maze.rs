use std::{fs::read_to_string, path::Path};

use cgmath::{perspective, vec3, Deg, EuclideanSpace, Matrix4, Point3};
use glfw::{Action, Key};
use learn_opengl::{
    camera::{Camera, CameraDirection, CameraDirectionTrait},
    gls::{
        buffers::{
            bindable::Bindable,
            texture::{Tex2DTrait, Texture2D, Textures},
            Attribute, VOs,
        },
        shader::{Shader, ShaderProgram},
    },
    lights::material::Material,
    window::Window,
};

use maze::logic::{Maze, MazeEntry, MazeIndex};
const SCR_WIDTH: u32 = 1600;
const SCR_HEIGHT: u32 = 1200;

const VERTEX_SHADER_SOURCE: &'static str = include_str!("../../shader/vert.glsl");
const FRAGMENT_SHADER_SOURCE: &'static str = include_str!("../../shader/frag.glsl");

const VERTEX_SHADER_SOURCE_LAMP: &'static str = include_str!("../../shader/lamp_vert.glsl");
const FRAGMENT_SHADER_SOURCE_LAMP: &'static str = include_str!("../../shader/lamp_frag.glsl");

const VERTEX_SHADER_SOURCE_DEPTH: &'static str = include_str!("../../shader/depth_vs.glsl");
const FRAGMENT_SHADER_SOURCE_DEPTH: &'static str = include_str!("../../shader/depth_fs.glsl");
const GEOMETRY_SHADER_SOURCE_DEPTH: &'static str = include_str!("../../shader/depth_ge.glsl");

fn main() {
    let mut window = Window::new(SCR_WIDTH, SCR_HEIGHT, "Learn Opengl", false, false).unwrap();
    window.window.set_framebuffer_size_polling(true);
    let mut maze: Maze = read_to_string("./maze.txt")
        .expect("Failed to find file maze.txt")
        .parse()
        .expect("Error parsing maze");

    let shader = ShaderProgram::new([
        Shader::new(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER).expect("Failed to Compile V Shader"),
        Shader::new(FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER)
            .expect("Failed to Compile F Normal Shader"),
    ])
    .expect("Failed to Create Shader Program");
    let lamp_shader = ShaderProgram::new([
        Shader::new(VERTEX_SHADER_SOURCE_LAMP, gl::VERTEX_SHADER)
            .expect("Failed to Compile V Shader"),
        Shader::new(FRAGMENT_SHADER_SOURCE_LAMP, gl::FRAGMENT_SHADER)
            .expect("Failed to Compile F Lamp Shader"),
    ])
    .expect("Failed to Create Shader Program");

    let depth_shader = ShaderProgram::new([
        Shader::new(VERTEX_SHADER_SOURCE_DEPTH, gl::VERTEX_SHADER)
            .expect("Failed to Compile V Shader"),
        Shader::new(FRAGMENT_SHADER_SOURCE_DEPTH, gl::FRAGMENT_SHADER)
            .expect("Failed to Compile F Depth Shader"),
        Shader::new(GEOMETRY_SHADER_SOURCE_DEPTH, gl::GEOMETRY_SHADER)
            .expect("Failed to Compile G Shader"),
    ])
    .expect("Failed to Create Shader Program Depth");

    #[rustfmt::skip]
    let cube_verts: [f32; 288] = [
        // positions       // normals        // texture coords
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  0.0,
         0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  0.0,
         0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  1.0,
         0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  1.0,
        -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  1.0,
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  0.0,

        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  0.0,
         0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  0.0,
         0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  1.0,
         0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  1.0,
        -0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  1.0,
        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  0.0,

        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0,  0.0,
        -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,  1.0,  1.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0,  1.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0,  1.0,
        -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,  0.0,  0.0,
        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0,  0.0,

         0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0,  0.0,
         0.5,  0.5, -0.5,  1.0,  0.0,  0.0,  1.0,  1.0,
         0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0,  1.0,
         0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0,  1.0,
         0.5, -0.5,  0.5,  1.0,  0.0,  0.0,  0.0,  0.0,
         0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0,  0.0,

        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0,  1.0,
         0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  1.0,  1.0,
         0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0,  0.0,
         0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0,  0.0,
        -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  0.0,  0.0,
        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0,  1.0,

        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  1.0,
         0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  1.0,  1.0,
         0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0,  0.0,
         0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0,  0.0,
        -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  0.0,  0.0,
        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  1.0
        ];
    let attributes = [
        Attribute {
            // cords
            location: 0,
            size: 3,
            normalized: false,
            stride: 8,
            offset: 0,
        },
        Attribute {
            // normal
            location: 1,
            size: 3,
            normalized: false,
            stride: 8,
            offset: 3,
        },
        Attribute {
            // texture
            location: 2,
            size: 2,
            normalized: false,
            stride: 8,
            offset: 6,
        },
    ];
    let vbo_vba =
        VOs::new(&cube_verts, &attributes, gl::TRIANGLES).expect("vbo or vba failed to bind");

    const SHADOW_WIDTH: i32 = 1024;
    const SHADOW_HEIGHT: i32 = 1024;

    shader
        .set_uniform("material", Material::new(0, 1, 0.5))
        .expect("error with material");

    shader.set_uniform("pointLight", maze.get_lights()).unwrap();

    if maze.get_lights().len() > 10 {
        panic!("There are too many lights: value must be less than 10")
    }

    shader
        .set_uniform("light_num", maze.get_lights().len() as i32)
        .unwrap();

    let mut cam = Camera::new(
        Point3::from_vec(maze.get_player_loc()),
        90f32,
        0f32,
        vec3(2.5, 2.5, 2.5),
    );
    shader.set_uniform("viewPos", cam.get_pos()).unwrap();

    let mut projection: Matrix4<f32> =
        perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);

    shader.set_uniform("projection", projection).unwrap();
    lamp_shader.set_uniform("projection", projection).unwrap();

    let near_plane = 1.0f32;
    let far_plane = 10.0f32;
    shader.set_uniform("far_plane", far_plane).unwrap();
    depth_shader.set_uniform("far_plane", far_plane).unwrap();

    // textures
    let floor_textures = [
        Texture2D::new(
            image::open(&Path::new("./maze/resources/floor/albedo.jpg")).unwrap(),
            [gl::REPEAT, gl::REPEAT],
            [gl::LINEAR, gl::LINEAR],
            gl::RGB,
            None,
        )
        .unwrap(),
        Texture2D::new(
            image::open(&Path::new("./maze/resources/floor/specular.jpg")).unwrap(),
            [gl::REPEAT, gl::REPEAT],
            [gl::LINEAR, gl::LINEAR],
            gl::RGB,
            None,
        )
        .unwrap(),
    ];

    let mut textures: Vec<&dyn Tex2DTrait> = vec![&floor_textures[0], &floor_textures[1]];
    for point_light in maze.get_lights().iter() {
        textures.push(point_light.get_cube_map());
    }
    let textures = Textures::new(textures.as_slice()).unwrap();
    textures.bind().unwrap();

    window.app_loop(|mut w| {
        process_events(&mut w, &mut cam, &mut projection);
        let mut show_lamps = false;
        let dir = process_input(&mut w.window, &mut show_lamps);
        if let Some(dir) = dir {
            if dir != 0 {
                let time = w.delta_time;
                let new_pos: MazeIndex = cam.try_translate_camera(dir, time).into();
                if maze[new_pos] != MazeEntry::Wall {
                    cam.translate_camera(dir, time);
                    shader.set_uniform("viewPos", cam.get_pos()).unwrap();
                    maze.move_player_to(new_pos);
                }
            }
        }
        let view = cam.get_view();
        for point_light in maze.get_lights().iter() {
            // 1. render scene to depth cubemap
            //
            depth_shader
                .set_uniform("lightPos", point_light.get_pos())
                .unwrap();
            depth_shader
                .set_uniform(
                    "shadowMatrices",
                    point_light.get_shadow_proj(1024., 1024., near_plane, far_plane),
                )
                .unwrap();
            unsafe {
                gl::Viewport(0, 0, SHADOW_WIDTH, SHADOW_HEIGHT);
                point_light.get_fbo().bind().unwrap();
                gl::Clear(gl::DEPTH_BUFFER_BIT);
            }

            depth_shader.use_program();
            render(&vbo_vba, &depth_shader, &maze);
        }
        // // 2. render normally
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::Viewport(0, 0, SCR_WIDTH as i32, SCR_HEIGHT as i32);
            gl::Clear(gl::DEPTH_BUFFER_BIT | gl::COLOR_BUFFER_BIT);
        }
        shader.set_uniform("view", view.clone()).unwrap();
        render(&vbo_vba, &shader, &maze);

        if show_lamps {
            lamp_shader.use_program();
            lamp_shader.set_uniform("view", view).unwrap();
            for point_light in maze.get_lights().iter() {
                lamp_shader
                    .set_uniform(
                        "model",
                        Matrix4::from_translation(point_light.get_pos()) * Matrix4::from_scale(0.2),
                    )
                    .unwrap();
                vbo_vba.draw_arrays(0, 36).unwrap();
            }
        }
    });
}

fn render(vbo_vba: &VOs, shader: &ShaderProgram, maze: &Maze) {
    for (y, row) in maze.iter().enumerate() {
        for (x, entry) in row.iter().enumerate() {
            // if entry == &MazeEntry::Wall {
            //     let model = Matrix4::from_translation(vec3(x as f32, 1., y as f32));
            //     shader.set_uniform("model", model).unwrap();
            //     vbo_vba.draw_arrays(0, 36).unwrap();
            // }
            let model: Matrix4<f32> = match entry {
                maze::logic::MazeEntry::Wall => {
                    Matrix4::from_translation(vec3(x as f32, 1., y as f32))
                    // shader
                    //     .set_uniform("model", model * Matrix4::from_translation(vec3(0., 1., 0.)))
                    //     .unwrap();
                    // vbo_vba.draw_arrays(0, 36).unwrap();
                    // model
                }
                _ => {
                    let model = Matrix4::from_translation(vec3(x as f32, 0., y as f32));
                    shader
                        .set_uniform("model", model * Matrix4::from_translation(vec3(0., 2., 0.)))
                        .unwrap();
                    vbo_vba.draw_arrays(0, 36).unwrap();
                    model
                }
            };
            shader.set_uniform("model", model).unwrap();
            vbo_vba.draw_arrays(0, 36).unwrap();
        }
    }
}

fn process_events(w: &mut Window, cam: &mut Camera, proj: &mut Matrix4<f32>) -> bool {
    for (_, event) in glfw::flush_messages(&w.events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                w.width = width as u32;
                w.height = height as u32;
                *proj = perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
                unsafe {
                    gl::Viewport(0, 0, width, height);
                };
            }
            glfw::WindowEvent::CursorPos(x, y) => {
                cam.move_point_pos(x as f32, y as f32);
            }
            _ => {}
        };
    }
    return false;
}
fn process_input(window: &mut glfw::Window, show_lamps: &mut bool) -> Option<CameraDirection> {
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true);
        return None;
    }
    let mut dirs = CameraDirection::new();

    if window.get_key(Key::W) == Action::Press {
        dirs.toggle_forward();
    }

    if window.get_key(Key::S) == Action::Press {
        dirs.toggle_backward();
    }

    if window.get_key(Key::D) == Action::Press {
        dirs.toggle_right();
    }

    if window.get_key(Key::A) == Action::Press {
        dirs.toggle_left();
    }

    // if window.get_key(Key::LeftShift) == Action::Press {
    //     dirs.toggle_down();
    // }

    if window.get_key(Key::Space) == Action::Press {
        *show_lamps = true;
    }
    // if window.get_key(Key::Space) == Action::Press {
    //     dirs.toggle_up();
    // }

    // if window.get_key(Key::T) == Action::Press {
    //     *prev_show_depth = !(*prev_show_depth);
    //     shader.set_uniform("show_depth", *prev_show_depth).unwrap();
    // }
    return Some(dirs);
}
