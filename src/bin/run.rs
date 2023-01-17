use glfw::{Action, Context, Key};
use learn_opengl::gls::buffers::{Attribute, VOs, EBO};
use learn_opengl::gls::shader::{Shader, ShaderProgram};

use learn_opengl::window;
use std::sync::mpsc::Receiver;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() {
       gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
       FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
"#;

fn main() {
    let mut glfw = window::init_glfw().expect("Failed to Initalize GLFW");
    let (mut window, events) = window::make_window(SCR_WIDTH, SCR_HEIGHT, "Learn Opengl", &glfw)
        .expect("Failed to start window");
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let v_shader =
        Shader::new(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER).expect("Failed to Compile V Shader");
    let f_shader = Shader::new(FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER)
        .expect("Failed to Compile F Shader");

    let shader = ShaderProgram::new([v_shader, f_shader]).expect("Failed to Create Shader Program");

    let verts: [f32; 12] = [
        0.5, 0.5, 0.0, // top right
        0.5, -0.5, 0.0, // bot right
        -0.5, -0.5, 0.0, // bot left
        -0.5, 0.5, 0.0, // top left
    ];

    let indices: [u32; 6] = [
        0, 1, 3, // T1
        1, 2, 3, // T2
    ];

    let attributes = [Attribute {
        location: 0,
        size: 3,
        normalized: false,
        offset: 0,
        stride: 3,
    }];
    let vbo_vba = VOs::new(&verts, &attributes, gl::TRIANGLES).expect("vbo or vba failed to bind");
    let ebo = EBO::new(&indices).expect("Failed to create Element Buffer");

    while !window.should_close() {
        // events
        // -----
        process_events(&mut window, &events);

        // render
        // ___

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        shader.use_program();
        // vbo_vba.draw_arrays(0, 3);
        ebo.draw_elements(&vbo_vba, 6, 0);

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
