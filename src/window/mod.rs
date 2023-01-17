use anyhow::{Context, Result};
use glfw::Context as _;
pub fn init_glfw() -> Result<glfw::Glfw> {
    glfw::init(glfw::FAIL_ON_ERRORS)
        .map(|mut glfw| {
            glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
            glfw.window_hint(glfw::WindowHint::OpenGlProfile(
                glfw::OpenGlProfileHint::Core,
            ));
            #[cfg(target_os = "macos")]
            glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
            glfw
        })
        .context("Couldn't Initalize glfw")
}
pub fn make_window(
    width: u32,
    height: u32,
    title: &str,
    glfw: &glfw::Glfw,
) -> Option<(
    glfw::Window,
    std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
)> {
    glfw.create_window(width, height, title, glfw::WindowMode::Windowed)
        .map(|(mut window, events)| {
            window.make_current();
            window.set_key_polling(true);
            window.set_framebuffer_size_polling(true);
            (window, events)
        })
}
