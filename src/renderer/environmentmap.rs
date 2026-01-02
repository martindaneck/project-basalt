use glam::{Mat4, Vec3};
use gltf::camera::Projection;
use crate::renderer::framebuffer;

use super::{TextureCube, LightCube, Shader, Framebuffer, Texture2D};

pub struct EnvironmentMap {
    cube: LightCube, // vertices for rendering purposes
    pub environment: TextureCube,
    pub irradiance: TextureCube,
}

impl EnvironmentMap {
    pub fn new(hdr_path: &str) -> Self {
        let cube = LightCube::new();
        let environment = TextureCube::from_hdr_equirectangular(hdr_path, 1024);
        let irradiance = Self::convolute_environment_map(&cube, &environment, 32);
        Self { cube, environment, irradiance }
    }

    pub fn draw_skybox(&self) {
        // Bind the environment texture
        self.environment.bind(0);

        // depth func and culling
        unsafe {
            gl::DepthFunc(gl::LEQUAL);
            gl::Disable(gl::CULL_FACE);
        }

        // cube
        self.cube.draw();

        // reset depth func and culling
        unsafe {
            gl::DepthFunc(gl::LESS);
            gl::Enable(gl::CULL_FACE);
        }
    }

    pub fn convolute_environment_map(cube: &LightCube, environment: &TextureCube, size: u32) -> TextureCube {
        let shader = Shader::from_files(
            "assets/shaders/cubemap.vertex.glsl",
            "assets/shaders/convolute_irradiance.fragment.glsl",
        );

        let irradiance_map = TextureCube::empty(size, gl::RGB16F, false);

        let mut temp_texture = Texture2D::empty(size, size, gl::RGB16F, gl::LINEAR, gl::CLAMP_TO_EDGE);
        let mut framebuffer = Framebuffer::new(size, size);
        framebuffer.add_color_attachment(temp_texture.clone());
        framebuffer.check_complete();

        // Perform convolution here...
        let projection = Mat4::perspective_rh_gl(90.0f32.to_radians(), 1.0, 0.01, 10.0);
        let views = [
            Mat4::look_at_rh(Vec3::ZERO, Vec3::X, -Vec3::Y),
            Mat4::look_at_rh(Vec3::ZERO, -Vec3::X, -Vec3::Y),
            Mat4::look_at_rh(Vec3::ZERO, Vec3::Y, Vec3::Z),
            Mat4::look_at_rh(Vec3::ZERO, -Vec3::Y, -Vec3::Z),
            Mat4::look_at_rh(Vec3::ZERO, Vec3::Z, -Vec3::Y),
            Mat4::look_at_rh(Vec3::ZERO, -Vec3::Z, -Vec3::Y),
        ];

        shader.bind();
        shader.set_int("environmentMap", 0);
        shader.set_mat4("projection", &projection);
        environment.bind(0);
        framebuffer.bind();
        unsafe {
            gl::Viewport(0, 0, size as i32, size as i32);
            gl::Disable(gl::CULL_FACE);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        }
        for i in 0..6 {
            shader.set_mat4("view", &views[i]);
            // clear temp texture
            unsafe {
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);

                // draw the cube
                cube.draw();

                // copy temporary texture to cubemap face
                gl::CopyImageSubData(
                    temp_texture.id,
                    gl::TEXTURE_2D,
                    0,
                    0,
                    0,
                    0,
                    irradiance_map.id,
                    gl::TEXTURE_CUBE_MAP,
                    0,
                    0,
                    0,
                    i as i32,
                    size as i32,
                    size as i32,
                    1,
                );
            }
        }
        unsafe {
            gl::Enable(gl::CULL_FACE);
        }

        irradiance_map
    }

    pub fn bind_irradiance(&self, unit: u32) {
        self.irradiance.bind(unit);
    }
}