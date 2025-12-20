#![allow(dead_code, unused)]

use glfw::{Action, Context, Key};
use glam::{self, Mat4};

mod renderer;
use renderer::shader::Shader;
use renderer::mesh::Mesh;
use renderer::app::App;

fn main() {
    let mut app = App::new(1920, 1080, "OpenGL Triangle");


    let shader = Shader::from_files(
        "assets/shaders/default.vertex.glsl",
        "assets/shaders/default.fragment.glsl",
    );

    let triangle = Mesh::triangle(Some("assets/textures/brickwall_texture/albedo.png"),
                                  None,
                                  None);

    while app.is_running() {
        app.begin_frame();
        app.update_shader_camera(&shader);

        shader.bind();

        let model_matrix = Mat4::from_translation(glam::vec3(0.0, 0.0, -2.0));
        shader.set_mat4("model", &model_matrix);
        triangle.draw();

        app.end_frame();
    }
}
