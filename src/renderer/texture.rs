use image::GenericImageView;
use imgui::internal;

pub enum TextureFormat {
    SrgbRGBA,
    LinearRGBA,
}

#[derive(Clone)]
pub struct Texture2D {
    pub id: u32,
    pub width: i32,
    pub height: i32,
}

impl Texture2D {
    pub fn from_file(path: &str, format: TextureFormat) -> Self {
        let img = image::open(path)
            .expect("Failed to open image")
            .flipv()
            .to_rgba8();

        let (width, height) = img.dimensions();
        
        let internal_format = match format {
            TextureFormat::SrgbRGBA => gl::SRGB8_ALPHA8,
            TextureFormat::LinearRGBA => gl::RGBA8,
        };

        let mut id = 0;

        unsafe {
            gl::CreateTextures(gl::TEXTURE_2D, 1, &mut id);
            gl::TextureStorage2D(id, 1, internal_format, width as i32, height as i32);
            gl::TextureSubImage2D(
                id,
                0,
                0,
                0,
                width as i32,
                height as i32,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img.as_ptr() as *const _,
            );

            gl::TextureParameteri(id, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TextureParameteri(id, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TextureParameteri(id, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
            gl::TextureParameteri(id, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::GenerateTextureMipmap(id);
        }

        Self { id, width: width as i32, height: height as i32 }
   }

   pub fn from_rgba8(color: [u8; 4]) -> Self {
        let mut id = 0;

        unsafe {
            gl::CreateTextures(gl::TEXTURE_2D, 1, &mut id);
            gl::TextureStorage2D(id, 1, gl::RGBA8, 1, 1);
            gl::TextureSubImage2D(
                id,
                0,
                0,
                0,
                1,
                1,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                color.as_ptr() as *const _,
            );

            gl::TextureParameteri(id, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TextureParameteri(id, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TextureParameteri(id, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TextureParameteri(id, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }

        Self { id, width: 1, height: 1 }
    }

    pub fn from_gltf(
        image: Option<&gltf::image::Data>,
        srgb: bool,
    ) -> Self {
        if let Some(img) = image {
            let (format, internal_format) = match img.format {
                gltf::image::Format::R8G8B8A8 => (
                    gl::RGBA,
                    if srgb { gl::SRGB8_ALPHA8 } else { gl::RGBA8 },
                ),
                gltf::image::Format::R8G8B8 => (
                    gl::RGB,
                    if srgb { gl::SRGB8 } else { gl::RGB8 },
                ),
                _ => panic!("Unsupported image format"),
            };

            let mut id = 0;

            unsafe {
                gl::CreateTextures(gl::TEXTURE_2D, 1, &mut id);
                gl::TextureStorage2D(id, 1, internal_format, img.width as i32, img.height as i32);
                gl::TextureSubImage2D(
                    id,
                    0,
                    0,
                    0,
                    img.width as i32,
                    img.height as i32,
                    format,
                    gl::UNSIGNED_BYTE,
                    img.pixels.as_ptr() as *const _,
                );

                gl::TextureParameteri(id, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
                gl::TextureParameteri(id, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
                gl::TextureParameteri(id, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
                gl::TextureParameteri(id, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
                gl::GenerateTextureMipmap(id);
            }
            Self { id, width: img.width as i32, height: img.height as i32 }
        } else {
            // default white texture
            DefaultTextures::new().normal
        }
    }

    pub fn empty(width: u32, height: u32, internal_format: u32, filter: u32, wrap_mode: u32) -> Self {
        let mut id = 0;

        unsafe {
            gl::CreateTextures(gl::TEXTURE_2D, 1, &mut id);
            gl::TextureStorage2D(id, 1, internal_format, width as i32, height as i32);
            gl::TextureParameteri(id, gl::TEXTURE_MIN_FILTER, filter as i32);
            gl::TextureParameteri(id, gl::TEXTURE_MAG_FILTER, filter as i32);
            gl::TextureParameteri(id, gl::TEXTURE_WRAP_S, wrap_mode as i32);
            gl::TextureParameteri(id, gl::TEXTURE_WRAP_T, wrap_mode as i32);
        }

        Self { id, width: width as i32, height: height as i32 }
    }

    pub fn bind(&self, unit: u32) {
        unsafe {
            gl::BindTextureUnit(unit, self.id);
        }
    }
}


pub struct DefaultTextures {
    pub white: Texture2D,
    pub normal: Texture2D,
    pub black: Texture2D,
}

impl DefaultTextures {
    pub fn new() -> Self {
        Self {
            white: Texture2D::from_rgba8([255, 255, 255, 255]),
            normal: Texture2D::from_rgba8([128, 128, 255, 255]),
            black: Texture2D::from_rgba8([0, 0, 0, 255]),
        }
    }
}