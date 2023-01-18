use glfw::{Action, Context, Key};
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

    let mut camera_pos: Vector3<f32> = vec3(0., 0., 3.);
    let mut camera_target: Vector3<f32> = vec3(0., 0., 0.);
    let up: Vector3<f32> = vec3(0., 1., 0.);

    while !window.should_close() {
        // events
        // -----
        process_events(&mut window, &events);
        // render
        // ___
        let radius = 10f32;
        camera_pos.x = radius * glfw.get_time().sin() as f32;
        camera_pos.z = radius * glfw.get_time().cos() as f32;
        let camera_direction = (camera_pos - camera_target).normalize();
        let camera_right = up.cross(camera_direction).normalize();
        let camera_up = camera_direction.cross(camera_right);
        let view = Matrix4::<f32>::look_at_rh(
            Point3::from_vec(camera_pos),
            Point3::from_vec(camera_target),
            camera_up,
        );

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // let model: Matrix4<f32> =
        // Matrix4::from_axis_angle(vec3(0.5, 1.0, 0.0), Rad(glfw.get_time() as f32));
        // let view: Matrix4<f32> = Matrix4::from_translation(vec3(0., 0., -3.));
        let projection: Matrix4<f32> =
            perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);

        // shader.set_uniform("model", model).unwrap();
        shader.set_uniform("view", view).unwrap();
        shader.set_uniform("projection", projection).unwrap();

        shader.use_program();
        let texs = Textures::new([&texture1, &texture2]).unwrap();
        texs.bind();

        for (i, pos) in cube_positions.iter().enumerate() {
            let mut model = Matrix4::from_translation(*pos);
            let angle = 20.0 * i as f32;
            model = model * Matrix4::from_axis_angle(vec3(1.0, 0.3, 0.5).normalize(), Deg(angle));
            shader.set_uniform("model", model).unwrap();
            vbo_vba.draw_arrays(0, 36);
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }
}
