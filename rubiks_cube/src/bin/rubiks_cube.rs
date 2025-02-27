use std::f32::consts::PI;
use std::{convert::TryInto, path::Path};

use cgmath::{perspective, vec3, vec4, Deg, Matrix4, Point3, Rad, Vector4};
use glfw::{Action, Key};
use learn_opengl::gls::buffers::texture::Tex2DTrait;
use learn_opengl::{
    gls::{
        buffers::{
            bindable::Bindable,
            texture::{Texture2D, Textures},
            Attribute, VOs,
        },
        shader::{Shader, ShaderProgram},
    },
    window::Window,
};

use rubiks_cube::{
    camera::Camera,
    game_logic::{RubiksCube, ShadowPlane},
};

const SCR_WIDTH: u32 = 1600;
const SCR_HEIGHT: u32 = 1200;

const VERTEX_SHADER_SOURCE: &'static str = include_str!("../../shaders/vert.glsl");
const FRAG_SHADER_SOURCE: &'static str = include_str!("../../shaders/frag.glsl");

fn main() {
    let mut window = Window::new(SCR_WIDTH, SCR_HEIGHT, "Rubiks Cube", true, false).unwrap();
    let v_shader =
        Shader::new(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER).expect("Failed to compile V Shader");
    let f_shader =
        Shader::new(FRAG_SHADER_SOURCE, gl::FRAGMENT_SHADER).expect("Failed to compile F Shader");
    let shader = ShaderProgram::new([v_shader, f_shader]).expect("Failed to Create Shader Program");

    #[rustfmt::skip]
    let face_verts: [f32; 30] = [
        -0.5, -0.5, 0.5, 0., 0.,
         0.5, -0.5, 0.5, 1., 0.,
         0.5,  0.5, 0.5, 1., 1.,
         0.5,  0.5, 0.5, 1., 1.,
        -0.5,  0.5, 0.5, 0., 1.,
        -0.5, -0.5, 0.5, 0., 0.,
    ];

    let attributes = [
        Attribute {
            // cords
            location: 0,
            size: 3,
            normalized: false,
            stride: 5,
            offset: 0,
        },
        Attribute {
            // texture
            location: 1,
            size: 2,
            normalized: false,
            stride: 5,
            offset: 3,
        },
    ];

    let imgs: [Texture2D; 12] = core::array::from_fn(|n| {
        let img = image::open(&Path::new(&format!("./rubiks_cube/resources/{}.png", n)))
            .expect(&format!("error with image {}", n));
        let img = match n {
            1 | 2 | 7 | 8 => img.fliph(),
            _ => img,
        };
        Texture2D::new(
            if n != 6 { img.flipv() } else { img },
            [gl::REPEAT, gl::REPEAT],
            [gl::LINEAR, gl::LINEAR],
            if n < 7 { gl::RGBA } else { gl::RGB },
            None,
        )
        .expect(&format!("error with image {}", n))
    });

    let win_img = image::open(&Path::new("./rubiks_cube/resources/win.jpeg")).unwrap();
    let win_texture = Texture2D::new(
        win_img.flipv(),
        [gl::REPEAT, gl::REPEAT],
        [gl::LINEAR, gl::LINEAR],
        gl::RGB,
        None,
    )
    .unwrap();

    let mut texs: Vec<&dyn Tex2DTrait> = imgs.iter().map(|x| x as &dyn Tex2DTrait).collect();
    texs.push(&win_texture);

    let texs = Textures::new(texs.as_slice()).unwrap();
    texs.bind().unwrap();

    shader.set_uniform("has_texture", false).unwrap();

    let face_obj =
        VOs::new(&face_verts, &attributes, gl::TRIANGLES).expect("vbo or vba failed to bind");

    let mut projection: Matrix4<f32> =
        perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
    shader.set_uniform("projection", projection).unwrap();
    let mut cam = Camera::new(Point3::new(1., 1., 10.), Point3::new(1., 1., 1.));

    let mut cube_state = if std::env::args().len() <= 1 {
        RubiksCube::new()
    } else {
        let val = std::env::args().last().expect("Should always pass");
        match val.parse::<usize>() {
            Ok(v) => {
                let mut c = RubiksCube::new();
                c.shuffle(v);
                c
            }
            Err(_) => RubiksCube::new_from_save(&val).expect("Invalid Input file"),
        }
    };

    let mut last_left = false;
    const ANIMATION_DURATION: f64 = 0.5;
    let mut is_animating = false;
    let mut start_time: f64 = 0.;
    let mut rotating_face: usize = 0;
    let mut is_clockwise: bool = true;
    let mut won: bool = false;
    window.app_loop(|mut w| {
        let (rotate_clicked, is_left_click, show_shadow_face, is_shift) =
            process_input(&w.window, &cube_state);
        process_events(&mut w, &mut projection, &mut cam, is_left_click, last_left);
        last_left = is_left_click;

        shader.set_uniform("view", cam.get_view()).unwrap();
        if is_animating {
            let current_time = w.glfw.get_time() - start_time;
            if current_time >= ANIMATION_DURATION {
                is_animating = false;
                cube_state.rotate(rotating_face, is_clockwise).unwrap();
                won = cube_state.check_win();
            } else {
                let shadow_plane_cords: ShadowPlane = rotating_face.try_into().unwrap();
                for (face, block) in cube_state.iter().enumerate() {
                    for (y, row) in block.iter().enumerate() {
                        for (x, color) in row.iter().enumerate() {
                            let is_shadow_plane = shadow_plane_cords
                                .plane
                                .iter()
                                .flat_map(|(f, cords, _)| cords.map(|(y, x)| (f, y, x)))
                                .find(|&(&sf, sy, sx)| face == sf && sy == y && sx == x)
                                .map(|_| true)
                                .unwrap_or(false);

                            let model = if face == rotating_face || is_shadow_plane {
                                if face == rotating_face {
                                    let black_space_spot = vec3(
                                        0.,
                                        0.,
                                        match face {
                                            0 | 2 | 1 => 1.,
                                            _ => -1.,
                                        },
                                    );
                                    let model = block.convert_cords(x as f32, y as f32)
                                        * cube_state
                                            .get_rotate_matrix(
                                                rotating_face,
                                                face,
                                                x as f32,
                                                y as f32,
                                                current_time / ANIMATION_DURATION,
                                                is_clockwise,
                                            )
                                            .unwrap()
                                        * block.get_rotation()
                                        * Matrix4::from_translation(black_space_spot);
                                    shader.set_uniform("model", model).unwrap();
                                    shader
                                        .set_uniform("uColor", vec4(0f32, 0., 0., 0.))
                                        .unwrap();
                                    face_obj.draw_arrays(0, 6).unwrap();
                                    let model = block.convert_cords(x as f32, y as f32)
                                        * block.get_rotation()
                                        * Matrix4::from_translation(black_space_spot);
                                    shader.set_uniform("model", model).unwrap();
                                    face_obj.draw_arrays(0, 6).unwrap();
                                }

                                block.convert_cords(x as f32, y as f32)
                                    * cube_state
                                        .get_rotate_matrix(
                                            rotating_face,
                                            face,
                                            x as f32,
                                            y as f32,
                                            current_time / ANIMATION_DURATION,
                                            is_clockwise,
                                        )
                                        .unwrap()
                                    * block.get_rotation()
                            } else {
                                block.convert_cords(x as f32, y as f32) * block.get_rotation()
                            };
                            shader.set_uniform("model", model).unwrap();
                            shader
                                .set_uniform::<Vector4<f32>>("uColor", color.into())
                                .unwrap();
                            face_obj.draw_arrays(0, 6).unwrap();
                        }
                    }
                }
                if rotating_face >= 6 {
                    shader
                        .set_uniform("uColor", vec4(0f32, 0., 0., 0.))
                        .unwrap();
                    let p = Rad(if is_clockwise { 1. } else { -1. }
                        * (current_time / ANIMATION_DURATION) as f32
                        * PI
                        / 2.);
                    let (model, rotate) = match rotating_face {
                        6 => {
                            let model = Matrix4::from_translation(vec3(1., 1., 1.));
                            let rotate = Matrix4::from_angle_z(p);
                            (model, rotate)
                        }
                        7 => {
                            let model = Matrix4::from_translation(vec3(1., 1., 1.))
                                * Matrix4::from_angle_x(Deg(90.));
                            let rotate = Matrix4::from_angle_z(-p);
                            (model, rotate)
                        }
                        8 => {
                            let model = Matrix4::from_translation(vec3(1., 1., 1.))
                                * Matrix4::from_angle_y(Deg(90.));
                            let rotate = Matrix4::from_angle_z(p);
                            (model, rotate)
                        }
                        _ => unreachable!("can't get here"),
                    };

                    let left = model
                        * Matrix4::from_translation(vec3(0., 0., -1.))
                        * Matrix4::from_scale(3.);
                    let right = model
                        * Matrix4::from_translation(vec3(0., 0., -2.))
                        * Matrix4::from_scale(3.);
                    shader.set_uniform("model", left).unwrap();
                    face_obj.draw_arrays(0, 6).unwrap();
                    let left_spin = left * rotate;
                    shader.set_uniform("model", left_spin).unwrap();
                    face_obj.draw_arrays(0, 6).unwrap();
                    shader.set_uniform("model", right).unwrap();
                    face_obj.draw_arrays(0, 6).unwrap();
                    let right_spin = right * rotate;
                    shader.set_uniform("model", right_spin).unwrap();
                    face_obj.draw_arrays(0, 6).unwrap();
                }
                return;
            }
        }

        if let Some(face) = rotate_clicked {
            start_time = w.glfw.get_time();
            is_animating = true;
            rotating_face = face;
            is_clockwise = !is_shift;
            return;
        }

        for (face, block) in cube_state.iter().enumerate() {
            for (y, row) in block.iter().enumerate() {
                for (x, color) in row.iter().enumerate() {
                    let model = block.convert_cords(x as f32, y as f32) * block.get_rotation();
                    shader.set_uniform("model", model).unwrap();
                    shader
                        .set_uniform::<Vector4<f32>>("uColor", color.into())
                        .unwrap();
                    if y == 1 && x == 1 {
                        let face_tex = if show_shadow_face { face + 6 } else { face };
                        shader.set_uniform("has_texture", true).unwrap();
                        if won {
                            shader.set_uniform("ourTexture", 12).unwrap();
                        } else {
                            shader.set_uniform("ourTexture", face_tex as i32).unwrap();
                        }
                    }
                    face_obj.draw_arrays(0, 6).unwrap();
                    shader.set_uniform("has_texture", false).unwrap();
                }
            }
        }
    });
}

fn process_input(window: &glfw::Window, cube: &RubiksCube) -> (Option<usize>, bool, bool, bool) {
    let mut num: Option<usize> = None;

    if window.get_key(Key::Num0) == Action::Press {
        num = Some(0);
    } else if window.get_key(Key::Num1) == Action::Press {
        num = Some(1);
    } else if window.get_key(Key::Num2) == Action::Press {
        num = Some(2);
    } else if window.get_key(Key::Num3) == Action::Press {
        num = Some(3);
    } else if window.get_key(Key::Num4) == Action::Press {
        num = Some(4);
    } else if window.get_key(Key::Num5) == Action::Press {
        num = Some(5);
    } else if window.get_key(Key::Num6) == Action::Press {
        num = Some(6);
    } else if window.get_key(Key::Num7) == Action::Press {
        num = Some(7);
    } else if window.get_key(Key::Num8) == Action::Press {
        num = Some(8);
    }
    if window.get_key(Key::W) == Action::Press {
        cube.save("rubiks_cube_save.txt").unwrap();
    }

    return (
        num,
        window.get_mouse_button(glfw::MouseButton::Button1) == Action::Press,
        window.get_mouse_button(glfw::MouseButton::Button2) == Action::Press
            || window.get_key(Key::S) == Action::Press,
        window.get_key(Key::LeftShift) == Action::Press
            || window.get_key(Key::RightShift) == Action::Press,
    );
}

fn process_events(
    w: &mut Window,
    proj: &mut Matrix4<f32>,
    cam: &mut Camera,
    is_left_click: bool,
    last_left: bool,
) {
    glfw::flush_messages(&w.events)
        .into_iter()
        .for_each(|(_, event)| {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    // make sure th viewport matches the new window dimensions; note that width and
                    // height will be significantly larger than specified on retina displays.
                    w.width = width as u32;
                    w.height = height as u32;
                    *proj =
                        perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
                    unsafe {
                        gl::Viewport(0, 0, width, height);
                    };
                }
                glfw::WindowEvent::CursorPos(x, y) => {
                    if is_left_click && last_left {
                        cam.pan(x as f32, y as f32, w.delta_time)
                    } else if is_left_click && !last_left {
                        cam.set_last(x as f32, y as f32);
                    }
                }
                _ => {}
            };
        });
}
