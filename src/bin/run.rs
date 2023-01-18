use glfw::{Action, Context, Key};
use learn_opengl::camera::{Camera, CameraDirection, CameraDirectionTrait};
use learn_opengl::gls::buffers::texture::{Texture2D, Textures};
use learn_opengl::gls::buffers::{ebo::EBO, Attribute, Bindable, VOs};
use learn_opengl::gls::shader::{Shader, ShaderProgram};
use learn_opengl::window;
use std::path::Path;
use std::sync::mpsc::Receiver;

use cgmath::{perspective, vec3, Deg, EuclideanSpace, InnerSpace, Matrix4, Point3, Rad, Vector3};

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

const FRAGMENT_SHADER_SOURCE: &'static str = r#"
#version 330 core
out vec4 FragColor;

in vec3 ourColor;
in vec2 TexCoord;

// texture samplers
uniform sampler2D texture1;
uniform sampler2D texture2;

void main()
{
	// linearly interpolate between both textures (80% container, 20% awesomeface)
	FragColor = mix(texture(texture1, TexCoord), texture(texture2, TexCoord), 0.2);
}"#;
// FragColor = mix(texture(texture1, TexCoord), texture(texture2, TexCoord), 0.2);

const VERTEX_SHADER_SOURCE: &'static str = r#"
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;

out vec3 ourColor;
out vec2 TexCoord;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
	gl_Position = projection * view * model * vec4(aPos, 1.0);
	TexCoord = vec2(aTexCoord.x, aTexCoord.y);
}
"#;

fn main() {
    let mut glfw = window::init_glfw().expect("Failed to Initalize GLFW");

    let mut delta_time: f32;
    let mut last_frame: f32 = 0.;

    let (mut window, events) = window::make_window(SCR_WIDTH, SCR_HEIGHT, "Learn Opengl", &glfw)
        .expect("Failed to start window");
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    let v_shader =
        Shader::new(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER).expect("Failed to Compile V Shader");
    let f_shader = Shader::new(FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER)
        .expect("Failed to Compile F Shader");
    let shader = ShaderProgram::new([v_shader, f_shader]).expect("Failed to Create Shader Program");

    let verts: [f32; 180] = [
        -0.5, -0.5, -0.5, 0.0, 0.0, 0.5, -0.5, -0.5, 1.0, 0.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5, 0.5,
        -0.5, 1.0, 1.0, -0.5, 0.5, -0.5, 0.0, 1.0, -0.5, -0.5, -0.5, 0.0, 0.0, -0.5, -0.5, 0.5,
        0.0, 0.0, 0.5, -0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 1.0, 0.5, 0.5, 0.5, 1.0, 1.0, -0.5,
        0.5, 0.5, 0.0, 1.0, -0.5, -0.5, 0.5, 0.0, 0.0, -0.5, 0.5, 0.5, 1.0, 0.0, -0.5, 0.5, -0.5,
        1.0, 1.0, -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, -0.5, 0.5, 0.0,
        0.0, -0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5,
        -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, 0.5, 0.0, 0.0, 0.5, 0.5, 0.5,
        1.0, 0.0, -0.5, -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, -0.5, 1.0, 1.0, 0.5, -0.5, 0.5, 1.0, 0.0,
        0.5, -0.5, 0.5, 1.0, 0.0, -0.5, -0.5, 0.5, 0.0, 0.0, -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, 0.5,
        -0.5, 0.0, 1.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0,
        -0.5, 0.5, 0.5, 0.0, 0.0, -0.5, 0.5, -0.5, 0.0, 1.0,
    ];
    let cube_positions = [
        vec3(0.0, 0.0, 0.0),
        vec3(2.0, 5.0, -15.0),
        vec3(-1.5, -2.2, -2.5),
        vec3(-3.8, -2.0, -12.3),
        vec3(2.4, -0.4, -3.5),
        vec3(-1.7, 3.0, -7.5),
        vec3(1.3, -2.0, -2.5),
        vec3(1.5, 2.0, -2.5),
        vec3(1.5, 0.2, -1.5),
        vec3(-1.3, 1.0, -1.5),
    ];

    let attributes = [
        Attribute {
            location: 0,
            size: 3,
            normalized: false,
            stride: 5,
            offset: 0,
        },
        Attribute {
            location: 1,
            size: 2,
            normalized: false,
            stride: 5,
            offset: 3,
        },
    ];
    let vbo_vba = VOs::new(&verts, &attributes, gl::TRIANGLES).expect("vbo or vba failed to bind");

    let container_texture = image::open(&Path::new("container.jpg")).unwrap();
    let texture1 = Texture2D::new(
        container_texture,
        [gl::REPEAT, gl::REPEAT],
        [gl::LINEAR, gl::LINEAR],
        gl::RGB,
        None,
    )
    .expect("Failed to lode texture");

    let face_texture = image::open(&Path::new("awesomeface.png")).unwrap();
    let texture2 = Texture2D::new(
        face_texture.flipv(),
        [gl::REPEAT, gl::REPEAT],
        [gl::LINEAR, gl::LINEAR],
        gl::RGBA,
        None,
    )
    .expect("Failed to lode texture");

    shader.set_uniform("texture1", 0).unwrap();
    shader.set_uniform("texture2", 1).unwrap();

    // let mut camera_pos: Vector3<f32> = vec3(0., 0., 3.);
    // let mut camera_target: Vector3<f32> = vec3(0., 0., 0.);
    // let up: Vector3<f32> = vec3(0., 1., 0.);

    let mut cam = Camera::new(
        Point3::<f32>::new(0., 0., 10.),
        Point3::<f32>::new(0., 0., 0.),
        vec3(2.5, 2.5, 2.5),
    );
    while !window.should_close() {
        // events
        // -----
        process_events(&mut window, &events);
        let current_frame = glfw.get_time() as f32;
        delta_time = current_frame - last_frame;
        last_frame = current_frame;

        let dir = process_input(&mut window);
        if let Some(dir) = dir {
            if dir != 0 {
                cam.translate_camera(dir, delta_time);
            }
        }

        // render
        // ___
        // let radius = 10f32;
        // cam.set_x(radius * glfw.get_time().sin() as f32);
        // cam.set_z(radius * glfw.get_time().cos() as f32);

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        let view = cam.get_view();
        let projection: Matrix4<f32> =
            perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);

        // shader.set_uniform("model", model).unwrap();
        shader.set_uniform("view", view).unwrap();
        shader.set_uniform("projection", projection).unwrap();

        shader.use_program();
        let texs = Textures::new([&texture1, &texture2]).unwrap();
        texs.bind();

        for (_i, pos) in cube_positions.iter().enumerate() {
            let model = Matrix4::from_translation(*pos);
            // model = model
            //     * Matrix4::from_axis_angle(
            //         vec3(1.0, 0.3, 0.5).normalize(),
            //         Rad(glfw.get_time() as f32),
            //     );
            shader.set_uniform("model", model).unwrap();
            vbo_vba.draw_arrays(0, 36);
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_input(window: &mut glfw::Window) -> Option<CameraDirection> {
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

    if window.get_key(Key::Space) == Action::Press {
        dirs.toggle_up();
    }

    if window.get_key(Key::Tab) == Action::Press {
        dirs.toggle_down();
    }

    return Some(dirs);
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            // glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            //     window.set_should_close(true)
            // }
            _ => {}
        }
    }
}
