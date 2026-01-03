#![allow(dead_code, unused)]

use glfw::{Action, Context, Key};
use glam::{self, Mat4};
use image::codecs::hdr;

mod renderer;
use renderer::shader::Shader;
use renderer::mesh::{Mesh, FullscreenQuad, LightCube};
use renderer::model::Model;
use renderer::ubo_manager::UboManager;
use renderer::texture::Texture2D;
use renderer::hdr_pass::HdrPass;
use renderer::light::LightManager;
use renderer::environmentmap::EnvironmentMap;
mod app;
use app::App;
use app::imgui_settings::ImguiSettings;



fn main() {
    std::env::set_current_dir(env!("CARGO_MANIFEST_DIR")).expect("Failed to set CWD");

    let mut app = App::new(1920, 1080, "OpenGL Triangle"); // these numbers shouldn't really matter

    let mut ubo_manager = UboManager::new();

    let mut imgui_settings = ImguiSettings::new();

    let mut light_manager = LightManager::new();

    // environment map do
    let mut environment_map_fireplace = EnvironmentMap::new("assets/textures/fireplace_4k.hdr");
    let mut environment_map_sky = EnvironmentMap::new("assets/textures/sky_4k.hdr");

    // shaders
    let shader = Shader::from_files(
        "assets/shaders/default.vertex.glsl",
        "assets/shaders/default.fragment.glsl",
    );
    let tonemap_shader = Shader::from_files(
        "assets/shaders/quad.vertex.glsl",
        "assets/shaders/tonemap.fragment.glsl",
    );
    let lightindicator_shader = Shader::from_files(
        "assets/shaders/default.vertex.glsl",
        "assets/shaders/lightindicator.fragment.glsl",
    );
    let skybox_shader = Shader::from_files(
        "assets/shaders/skybox.vertex.glsl",
        "assets/shaders/skybox.fragment.glsl",
    );
    // meshes
    let fullscreen_quad = FullscreenQuad::new();

    let triangle = Mesh::triangle(Some("assets/textures/brickwall_texture/albedo.png"),
                                  Some("assets/textures/brickwall_texture/normal.png"),
                                  Some("assets/textures/brickwall_texture/orm.png"));
    let lightcube = LightCube::new();
    // models
    let amongus = Model::load("assets/models/amongusclay/scene.gltf");    
    // lights
    light_manager.add_light([0.0; 3], 0.0, [0.0; 3], 0.0); // initialize with zeros, first light is reserved for imgui
    // passes
    let mut hdr_pass = HdrPass::new(app.width as u32, app.height as u32);

    while app.is_running() {
        app.begin_frame();
        //imgui 
        imgui_settings.begin_frame(&mut app.window);
        imgui_settings.draw(app.get_fps());
        // update environment map
        let environment_map = if imgui_settings.get_settings().2 == 0 {
            &mut environment_map_fireplace
        } else {
            &mut environment_map_sky
        };
        // update lights
        light_manager.set_light(0, imgui_settings.get_light());
        // update UBOs
        ubo_manager.set_settings(imgui_settings.get_settings());
        ubo_manager.set_camera(app.get_view_projection_position());
        ubo_manager.set_lights(light_manager.get_lights());
        ubo_manager.update();
        
        /// HDR pass
        hdr_pass.begin(app.width as u32, app.height as u32);
        shader.bind();
        // bind environment maps
        environment_map.bind_irradiance(3);
        environment_map.bind_prefiltered(4);
        environment_map.bind_brdf_lut(5);

        // triangle
        let model_matrix = Mat4::from_translation(glam::vec3(0.0, 0.0, -2.0));
        shader.set_mat4("model", &model_matrix);
        triangle.draw();

        // amongus
        let model_matrix = Mat4::from_rotation_translation(
            glam::Quat::from_axis_angle(glam::Vec3::X, -90.0_f32.to_radians()),
            glam::vec3(-1.0, 0.0, 0.0),
        );
        shader.set_mat4("model", &model_matrix);
        amongus.draw();

        // draw light indicators
        lightindicator_shader.bind();
        let light_positions = light_manager.get_light_positions();
        for pos in light_positions.iter() {
            let model_matrix = Mat4::from_translation(
                glam::vec3(pos[0], pos[1], pos[2]),
            );
            lightindicator_shader.set_mat4("model", &model_matrix);
            lightcube.draw();
        }

        


        // draw skybox
        skybox_shader.bind();
        environment_map.draw_skybox();

        hdr_pass.end();

        /// tonemap pass
        tonemap_shader.bind();
        hdr_pass.framebuffer.color[0].bind(0);
        fullscreen_quad.draw();


        imgui_settings.end_frame();
        app.end_frame();
    }
}
