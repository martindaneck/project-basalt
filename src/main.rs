#![allow(dead_code, unused)]

use glfw::{Action, Context, Key};
use glam::{self, Mat4};

mod renderer;
use renderer::shader::Shader;
use renderer::mesh::Mesh;
use renderer::model::Model;
mod app;
use app::app::App;

fn main() {
    let mut app = App::new(1920, 1080, "OpenGL Triangle");


    let shader = Shader::from_files(
        "assets/shaders/default.vertex.glsl",
        "assets/shaders/default.fragment.glsl",
    );

    let triangle = Mesh::triangle(Some("assets/textures/brickwall_texture/albedo.png"),
                                  Some("assets/textures/brickwall_texture/normal.png"),
                                  Some("assets/textures/brickwall_texture/orm.png"));

    let amongus = Model::load("assets/models/amongusclay/scene.gltf");

    while app.is_running() {
        app.begin_frame();
        app.update_shader_camera(&shader);

        shader.bind();

        let model_matrix = Mat4::from_translation(glam::vec3(0.0, 0.0, -2.0));
        shader.set_mat4("model", &model_matrix);
        triangle.draw();

        let model_matrix = Mat4::from_rotation_translation(
            glam::Quat::from_axis_angle(glam::Vec3::X, 270.0_f32.to_radians()),
            glam::vec3(-1.0, 0.0, 0.0),
        );
        shader.set_mat4("model", &model_matrix);
        amongus.draw();

        app.end_frame();
    }
}
