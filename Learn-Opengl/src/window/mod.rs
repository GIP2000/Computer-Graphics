use anyhow::{Context, Result};
use glfw::Context as _;
use std::sync::mpsc::Receiver;

pub struct Window {
    pub glfw: glfw::Glfw,
    pub window: glfw::Window,
    pub events: Receiver<(f64, glfw::WindowEvent)>,
    pub width: u32,
    pub height: u32,
    pub delta_time: f32,
    last_frame: f32,
}

impl Window {
    pub fn new(
        width: u32,
        height: u32,
        title: &str,
        show_cursor: bool,
        is_full_screen: bool,
    ) -> Result<Self> {
        let mut glfw = Self::init_glfw()?;
        let (mut window, events) =
            Self::make_window(width, height, title, &mut glfw, show_cursor, is_full_screen)
                .context("Failed to make window")?;
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }
        Ok(Self {
            glfw,
            window,
            events,
            width,
            height,
            delta_time: 0.,
            last_frame: 0.,
        })
    }

    fn init_glfw() -> Result<glfw::Glfw> {
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

    fn make_window(
        width: u32,
        height: u32,
        title: &str,
        glfw: &mut glfw::Glfw,
        show_cursor: bool,
        is_full_screen: bool,
    ) -> Option<(glfw::Window, Receiver<(f64, glfw::WindowEvent)>)> {
        glfw.with_primary_monitor(|glfw, m| {
            let mut window_mode = glfw::WindowMode::Windowed;
            if is_full_screen {
                if let Some(m) = m {
                    window_mode = glfw::WindowMode::FullScreen(m);
                }
            }

            glfw.create_window(width, height, title, window_mode)
                .map(|(mut window, events)| {
                    window.make_current();
                    window.set_key_polling(true);
                    window.set_framebuffer_size_polling(true);
                    window.set_cursor_pos_polling(true);
                    window.set_cursor_mode(if show_cursor {
                        glfw::CursorMode::Normal
                    } else {
                        glfw::CursorMode::Disabled
                    });
                    (window, events)
                })
        })
    }
    pub fn app_loop<F>(&mut self, mut render_fn: F)
    where
        F: FnMut(&mut Self),
    {
        while !self.window.should_close() {
            let current_frame = self.glfw.get_time() as f32;
            self.delta_time = current_frame - self.last_frame;
            self.last_frame = current_frame;

            unsafe {
                gl::ClearColor(0.1, 0.1, 0.1, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            }
            render_fn(self);
            // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
            // -------------------------------------------------------------------------------
            self.window.swap_buffers();
            self.glfw.poll_events();
        }
    }
}
