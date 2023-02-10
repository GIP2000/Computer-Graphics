use cgmath::{perspective, vec3, vec4, Deg, Matrix4, Point3, Rad, Vector4};
use glfw::{Action, Key};
use learn_opengl::{
    gls::{
        buffers::{Attribute, VOs},
        shader::{Shader, ShaderProgram},
    },
    window::Window,
};
use rubiks_cube::game_logic::{Colors, RubiksCube};

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

const VERTEX_SHADER_SOURCE: &'static str = include_str!("../../shaders/vert.glsl");
const FRAG_SHADER_SOURCE: &'static str = include_str!("../../shaders/frag.glsl");

fn main() {
    println!("{}", VERTEX_SHADER_SOURCE);
    let mut window = Window::new(SCR_WIDTH, SCR_HEIGHT, "Rubiks Cube").unwrap();
    let v_shader =
        Shader::new(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER).expect("Failed to compile V Shader");
    let f_shader =
        Shader::new(FRAG_SHADER_SOURCE, gl::FRAGMENT_SHADER).expect("Failed to compile F Shader");
    let shader = ShaderProgram::new([v_shader, f_shader]).expect("Failed to Create Shader Program");
    let cube_verts: [f32; 144] = [
        -0.5, -0.5, -0.5, 1., 0.5, -0.5, -0.5, 1., 0.5, 0.5, -0.5, 1., 0.5, 0.5, -0.5, 1., -0.5,
        0.5, -0.5, 1., -0.5, -0.5, -0.5, 1., -0.5, -0.5, 0.5, 0., 0.5, -0.5, 0.5, 0., 0.5, 0.5,
        0.5, 0., 0.5, 0.5, 0.5, 0., -0.5, 0.5, 0.5, 0., -0.5, -0.5, 0.5, 0., -0.5, 0.5, 0.5, 2.,
        -0.5, 0.5, -0.5, 2., -0.5, -0.5, -0.5, 2., -0.5, -0.5, -0.5, 2., -0.5, -0.5, 0.5, 2., -0.5,
        0.5, 0.5, 2., 0.5, 0.5, 0.5, 3., 0.5, 0.5, -0.5, 3., 0.5, -0.5, -0.5, 3., 0.5, -0.5, -0.5,
        3., 0.5, -0.5, 0.5, 3., 0.5, 0.5, 0.5, 3., -0.5, -0.5, -0.5, 5., 0.5, -0.5, -0.5, 5., 0.5,
        -0.5, 0.5, 5., 0.5, -0.5, 0.5, 5., -0.5, -0.5, 0.5, 5., -0.5, -0.5, -0.5, 5., -0.5, 0.5,
        -0.5, 4., 0.5, 0.5, -0.5, 4., 0.5, 0.5, 0.5, 4., 0.5, 0.5, 0.5, 4., -0.5, 0.5, 0.5, 4.,
        -0.5, 0.5, -0.5, 4.,
    ];

    let attributes = [
        Attribute {
            // cords
            location: 0,
            size: 3,
            normalized: false,
            stride: 4,
            offset: 0,
        },
        Attribute {
            // face
            location: 1,
            size: 1,
            normalized: false,
            stride: 4,
            offset: 3,
        },
    ];
    let cube =
        VOs::new(&cube_verts, &attributes, gl::TRIANGLES).expect("vbo or vba failed to bind");

    let mut projection: Matrix4<f32> =
        perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
    shader.set_uniform("projection", projection).unwrap();
    let view: Matrix4<f32> = Matrix4::look_at_rh(
        Point3::new(0., 3., 5.),
        Point3::new(1., 1., 0.),
        vec3(0., 1., 0.),
    );
    shader.set_uniform("view", view).unwrap();

    // let mut cam = Camera::new(
    //     Point3::<f32>::new(0., 5., 2.),
    //     90f32,
    //     0f32,
    //     vec3(2.5, 2.5, 2.5),
    // );
    let mut cube_state = RubiksCube::new();
    let mut last_pressed = false;
    let mut last_release = false;
    window.app_loop(|mut w| {
        process_events(&mut w, &mut projection);

        shader.set_uniform("view", view).unwrap();

        for (i, block) in cube_state.iter().enumerate() {
            shader.set_uniform("uColor", block.get_colors()).unwrap();
            let y = i % 3;
            let x = (i / 3) % 3;
            let z = i / 9;
            let model: Matrix4<f32> =
                Matrix4::from_translation(vec3(x as f32, y as f32, -(z as f32)));

            shader.set_uniform("model", model).unwrap();
            cube.draw_arrays(0, 36).unwrap();
        }
    });
}

fn process_events(w: &mut Window, proj: &mut Matrix4<f32>) -> bool {
    glfw::flush_messages(&w.events)
        .into_iter()
        .for_each(|(_, event)| {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    // make sure the viewport matches the new window dimensions; note that width and
                    // height will be significantly larger than specified on retina displays.
                    w.width = width as u32;
                    w.height = height as u32;
                    *proj =
                        perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
                    unsafe {
                        gl::Viewport(0, 0, width, height);
                    };
                }
                glfw::WindowEvent::CursorPos(x, y) => {}
                _ => {}
            };
        });
    return false;
}
