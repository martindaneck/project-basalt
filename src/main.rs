#![allow(dead_code, unused)]

use glfw::{Action, Context, Key};
use glam::{self, Mat4};

mod renderer;
use renderer::shader::Shader;
use renderer::mesh::Mesh;
use renderer::model::Model;
use renderer::ubo_manager::UboManager;
mod app;
use app::App;
use app::imgui_settings::ImguiSettings;

fn main() {
    let mut app = App::new(1920, 1080, "OpenGL Triangle");

    let mut ubo_manager = UboManager::new();

    let mut imgui_settings = ImguiSettings::new();

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
        imgui_settings.begin_frame(&mut app.window);
        imgui_settings.draw();

        
        ubo_manager.set_settings(imgui_settings.get_settings());
        ubo_manager.set_camera(app.get_view_projection_position());
        ubo_manager.update();

        shader.bind();

        let model_matrix = Mat4::from_translation(glam::vec3(0.0, 0.0, -2.0));
        shader.set_mat4("model", &model_matrix);
        triangle.draw();

        let model_matrix = Mat4::from_rotation_translation(
            glam::Quat::from_axis_angle(glam::Vec3::X, -90.0_f32.to_radians()),
            glam::vec3(-1.0, 0.0, 0.0),
        );
        shader.set_mat4("model", &model_matrix);
        amongus.draw();

        
        
        imgui_settings.end_frame();
        
        app.end_frame();
    }
}
