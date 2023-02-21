use cgmath::{perspective, Deg, Matrix4, Point3, Vector4};
use glfw::{Action, Key};
use learn_opengl::{
    gls::{
        buffers::{Attribute, VOs},
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
    let mut window = Window::new(SCR_WIDTH, SCR_HEIGHT, "Rubiks Cube").unwrap();
    let v_shader =
        Shader::new(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER).expect("Failed to compile V Shader");
    let f_shader =
        Shader::new(FRAG_SHADER_SOURCE, gl::FRAGMENT_SHADER).expect("Failed to compile F Shader");
    let shader = ShaderProgram::new([v_shader, f_shader]).expect("Failed to Create Shader Program");
    let face_verts: [f32; 18] = [
        -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5, 0.5, -0.5, -0.5,
        0.5,
    ];

    let attributes = [Attribute {
        // cords
        location: 0,
        size: 3,
        normalized: false,
        stride: 3,
        offset: 0,
    }];
    let face_obj =
        VOs::new(&face_verts, &attributes, gl::TRIANGLES).expect("vbo or vba failed to bind");

    let mut projection: Matrix4<f32> =
        perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
    shader.set_uniform("projection", projection).unwrap();
    let mut cam = Camera::new(Point3::new(1., 1., 10.), Point3::new(1., 1., 1.));

    let mut cube_state = RubiksCube::new();
    let mut last_left = false;
    const ANIMATION_DURATION: f64 = 0.5;
    let mut is_animating = false;
    let mut start_time: f64 = 0.;
    let mut rotating_face: usize = 8;
    window.app_loop(|mut w| {
        let (rotate_clicked, is_left_click, is_right_click) = process_input(&w.window);
        process_events(
            &mut w,
            &mut projection,
            &mut cam,
            is_left_click,
            last_left,
            is_right_click,
        );
        last_left = is_left_click;

        shader.set_uniform("view", cam.get_view()).unwrap();
        if is_animating {
            let current_time = w.glfw.get_time() - start_time;
            if current_time >= ANIMATION_DURATION {
                is_animating = false;
                cube_state.rotate(rotating_face, true).unwrap();
            } else {
                let shadow_plane_cords: ShadowPlane = rotating_face.try_into().unwrap();
                for (face, block) in cube_state.iter().enumerate() {
                    for (y, row) in block.iter().enumerate() {
                        for (x, color) in row.iter().enumerate() {
                            let is_shadow_plane = shadow_plane_cords
                                .plane
                                .iter()
                                .flat_map(|(f, cords)| cords.map(|(y, x)| (f, y, x)))
                                .find(|&(&sf, sy, sx)| face == sf && sy == y && sx == x)
                                .map(|_| true)
                                .unwrap_or(false);

                            let model = if face == rotating_face || is_shadow_plane {
                                block.convert_cords(x as f32, y as f32)
                                    * cube_state
                                        .get_rotate_matrix(
                                            rotating_face,
                                            face,
                                            x as f32,
                                            y as f32,
                                            current_time / ANIMATION_DURATION,
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
                return;
            }
        }

        if rotate_clicked {
            start_time = w.glfw.get_time();
            is_animating = true;
            return;
        }

        for block in cube_state.iter() {
            for (y, row) in block.iter().enumerate() {
                for (x, color) in row.iter().enumerate() {
                    let model = block.convert_cords(x as f32, y as f32) * block.get_rotation();
                    shader.set_uniform("model", model).unwrap();
                    shader
                        .set_uniform::<Vector4<f32>>("uColor", color.into())
                        .unwrap();
                    face_obj.draw_arrays(0, 6).unwrap();
                }
            }
        }
    });
}
fn process_input(window: &glfw::Window) -> (bool, bool, bool) {
    (
        window.get_key(Key::Num3) == Action::Press,
        window.get_mouse_button(glfw::MouseButton::Button1) == Action::Press,
        window.get_mouse_button(glfw::MouseButton::Button2) == Action::Press,
    )
}

fn process_events(
    w: &mut Window,
    proj: &mut Matrix4<f32>,
    cam: &mut Camera,
    is_left_click: bool,
    last_left: bool,
    is_right_click: bool,
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
                    if is_right_click {
                        // calcualte world cords for x,y
                    }
                }
                _ => {}
            };
        });
}
