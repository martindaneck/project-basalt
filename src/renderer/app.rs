use glfw::{Action, Context, Key};

pub struct App {
    glfw: glfw::Glfw,
    window: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,

    width: i32,
    height: i32,
    resized: bool,
}

impl App {
    pub fn new(width: i32, height: i32, title: &str) -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));

        let (mut window, events) = glfw
            .create_window(width.try_into().unwrap(), height.try_into().unwrap(), title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        gl::load_with(|s| {
            window.get_proc_address(s)
                .map(|f| f as *const _)
                .unwrap_or(std::ptr::null())
        });

        let (width, height) = window.get_framebuffer_size();

        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }

        App { glfw, window, events, width, height, resized: false }
    }

    pub fn is_running(&self) -> bool {
        !self.window.should_close()
    }

    pub fn begin_frame(&mut self) {
        self.glfw.poll_events();

        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(w, h) => {
                    self.width = w;
                    self.height = h;
                    self.resized = true;

                    unsafe {
                        gl::Viewport(0, 0, w, h);
                    }
                }

                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true);
                }

                _ => {}
            }
        }


        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn end_frame(&mut self) {
        self.window.swap_buffers();
    }

    pub fn framebuffer_size(&self) -> (i32, i32) {
        (self.width, self.height)
    }

    pub fn was_resized(&mut self) -> bool {
        self.resized
    }

    pub fn clear_resized_flag(&mut self) {
        self.resized = false;
    }

}

