use glfw::{Action, Context, Key};
use std::ptr;

mod renderer;
use renderer::shader::Shader;

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw
        .create_window(800, 600, "Hello Triangle", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);

    gl::load_with(|s| {
        window.get_proc_address(s)
            .map(|f| f as *const _)
            .unwrap_or(std::ptr::null())
    });

    unsafe {
        gl::Viewport(0, 0, 800, 600);
    }

    let vertices: [f32; 9] = [
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0,  0.5, 0.0,
    ];

    let mut vao: u32 = 0;
    let mut vbo: u32 = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as isize,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * std::mem::size_of::<f32>()) as i32,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
    }

    let shader = Shader::from_files(
        "src/shaders/default.vertex.glsl",
        "src/shaders/default.fragment.glsl",
    );


    while !window.should_close() {
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                window.set_should_close(true);
            }
        }

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            shader.bind();
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }


        window.swap_buffers();
    }
}
