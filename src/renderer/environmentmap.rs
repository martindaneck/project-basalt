use glam::{Mat4, Vec3};
use gltf::camera::Projection;
use crate::renderer::framebuffer;

use super::{TextureCube, LightCube, Shader, Framebuffer, Texture2D, FullscreenQuad};

pub struct EnvironmentMap {
    cube: LightCube, // vertices for rendering purposes
    pub environment: TextureCube,
    pub irradiance: TextureCube,
    pub prefiltered: TextureCube,
    pub brdf_lut: Texture2D,
}

impl EnvironmentMap {
    pub fn new(hdr_path: &str) -> Self {
        let cube = LightCube::new();
        let environment = TextureCube::from_hdr_equirectangular(hdr_path, 1024);
        let irradiance = Self::convolute_environment_map(&cube, &environment, 32);
        let prefiltered = Self::prefilter_environment_map(&cube, &environment, 1024);
        let brdf_lut = Self::generate_brdf_lut();
        Self { cube, environment, irradiance, prefiltered, brdf_lut }
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

    pub fn bind_prefiltered(&self, unit: u32) {
        self.prefiltered.bind(unit);
    }

    pub fn bind_brdf_lut(&self, unit: u32) {
        self.brdf_lut.bind(unit);
    }

    pub fn prefilter_environment_map(cube: &LightCube, environment: &TextureCube, size: u32) -> TextureCube {
        let shader = Shader::from_files(
            "assets/shaders/cubemap.vertex.glsl",
            "assets/shaders/prefilter.fragment.glsl",
        );

        let prefiltered_map = TextureCube::empty(size, gl::RGB16F, true);

        let mut temp_texture = Texture2D::empty(size, size, gl::RGB16F, gl::LINEAR_MIPMAP_LINEAR, gl::CLAMP_TO_EDGE);

        let mut framebuffer = Framebuffer::new(size, size);
        framebuffer.add_color_attachment(temp_texture.clone());
        framebuffer.check_complete();

        // Perform prefiltering here...
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
        let max_mip_levels: u32 = (size as f32).log2().floor() as u32 + 1;
        for mip in 0..max_mip_levels {
            let mip_size = (size as f32 * 0.5_f32.powf(mip as f32)) as u32;
            framebuffer.recreate(mip_size, mip_size);
            let roughness = mip as f32 / (max_mip_levels as f32 - 1.0);
            shader.set_float("roughness", roughness);
            unsafe {
                gl::Viewport(0, 0, mip_size as i32, mip_size as i32);
                gl::Disable(gl::CULL_FACE);
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
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
                        prefiltered_map.id,
                        gl::TEXTURE_CUBE_MAP,
                        mip as i32,
                        0,
                        0,
                        i as i32,
                        mip_size as i32,
                        mip_size as i32,
                        1,
                    );
                }
            }
        };
        unsafe {
            gl::Enable(gl::CULL_FACE);
        }

        prefiltered_map
    }

    pub fn generate_brdf_lut() -> Texture2D {
        let size = 512;
        let shader = Shader::from_files(
            "assets/shaders/quad.vertex.glsl",
            "assets/shaders/brdf.fragment.glsl",
        );

        let brdf_lut = Texture2D::empty(size, size, gl::RG16F, gl::LINEAR, gl::CLAMP_TO_EDGE);

        let mut framebuffer = Framebuffer::new(size, size);
        framebuffer.add_color_attachment(brdf_lut.clone());
        framebuffer.check_complete();

        // Render BRDF LUT
        shader.bind();
        framebuffer.bind();
        unsafe {
            gl::Viewport(0, 0, size as i32, size as i32);
            gl::Disable(gl::CULL_FACE);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        // Render a fullscreen quad
        let quad = FullscreenQuad::new();
        quad.draw();
        unsafe {
            gl::Enable(gl::CULL_FACE);
        }

        brdf_lut
    }
}