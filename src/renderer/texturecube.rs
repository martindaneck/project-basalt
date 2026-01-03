use gltf::json::path;
use glam::{Mat4, Vec3};

use super::{Shader, Texture2D, Framebuffer, LightCube};

#[derive(Clone)]
pub struct TextureCube {
    pub id: u32,
    pub size: u32,
    pub internal_format: u32,
}

impl TextureCube {
    pub fn empty(size: u32, internal_format: u32, mipmapped: bool) -> Self {
        let mut id = 0;

        unsafe {
            gl::CreateTextures(gl::TEXTURE_CUBE_MAP, 1, &mut id);

            let levels = if mipmapped {
                (size as f32).log2().floor() as i32 + 1
            } else {
                1
            };

            gl::TextureStorage2D(
                id,
                levels,
                internal_format,
                size as i32,
                size as i32,
            );

            gl::TextureParameteri(id, gl::TEXTURE_MIN_FILTER, if mipmapped { gl::LINEAR_MIPMAP_LINEAR as i32 } else { gl::LINEAR as i32 });
            gl::TextureParameteri(id, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TextureParameteri(id, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TextureParameteri(id, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TextureParameteri(id, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);

            if mipmapped {
                gl::GenerateTextureMipmap(id);
            }

            Self {
                id,
                size,
                internal_format,
            }
        }
    }

    pub fn bind(&self, unit: u32) {
        unsafe {
            gl::BindTextureUnit(unit, self.id);
        }
    }

    pub fn from_hdr_equirectangular(path: &str, size: u32) -> Self {
        // Load the HDR equirectangular image
        let hdr_texture = Texture2D::from_hdr_file(path);
        // Create the cubemap
        let cubemap = TextureCube::empty(size, gl::RGB16F, true);
        // shader
        let shader = Shader::from_files(
            "assets/shaders/cubemap.vertex.glsl",
            "assets/shaders/equirectangulartocubemap.fragment.glsl",
        );
        // cube
        let cube = LightCube::new();
        // temporary texture
        let mut temp_texture = Texture2D::empty(size, size, gl::RGB16F, gl::NEAREST, gl::CLAMP_TO_EDGE);
        // Create the framebuffer
        let mut framebuffer = Framebuffer::new(size, size);
        framebuffer.add_color_attachment(temp_texture.clone());
        framebuffer.check_complete();

        // capture matrices
        let projection = Mat4::perspective_rh_gl(90.0f32.to_radians(), 1.0, 0.01, 10.0);
        let views = [
            Mat4::look_at_rh(Vec3::ZERO, Vec3::X, -Vec3::Y),
            Mat4::look_at_rh(Vec3::ZERO, -Vec3::X, -Vec3::Y),
            Mat4::look_at_rh(Vec3::ZERO, Vec3::Y, Vec3::Z),
            Mat4::look_at_rh(Vec3::ZERO, -Vec3::Y, -Vec3::Z),
            Mat4::look_at_rh(Vec3::ZERO, Vec3::Z, -Vec3::Y),
            Mat4::look_at_rh(Vec3::ZERO, -Vec3::Z, -Vec3::Y),
        ];

        // render
        shader.bind(); // bind the shader
        shader.set_int("equirectangularMap", 0); // set the equirectangular map
        shader.set_mat4("projection", &projection); // set the projection matrix
        hdr_texture.bind(0); // bind the HDR texture
        framebuffer.bind(); // bind the framebuffer
        unsafe {  // set viewport and disable culling and depth test, clear screen
            gl::Viewport(0, 0, size as i32, size as i32);
            gl::Disable(gl::CULL_FACE);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        }
        for i in 0..6 { // the six faces
            shader.set_mat4("view", &views[i]); // set the view matrix
            unsafe {
                // clear temporary texture
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);

                // draw the cube
                cube.draw();

                // copy temporary texture to cubemap face
                gl::CopyImageSubData( // NOTES TO SELF: IT IS ABSOLUTELY NECESSARY TO DRAW TO TEMPORARY TEXTURES FIRST AND COPY OVER TO THE CUBEMAP LATER,
                                      // OTHERWISE THE GRAPHICS DRIVERS THEMSELVES WILL PULL A DIPSHIT ON YOU AND CLEAR YOUR LAST FACE WITH GRAY
                    temp_texture.id,
                    gl::TEXTURE_2D,
                    0,
                    0,
                    0,
                    0,
                    cubemap.id,
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
        unsafe { // re-enable culling and depth test
            gl::Enable(gl::CULL_FACE);
            // generate mipmaps
            gl::GenerateTextureMipmap(cubemap.id);
        }
        // Create a new TextureCube
        cubemap
    }
}
