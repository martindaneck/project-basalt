use super::{Buffer, VertexArray, Texture2D, TextureFormat, DefaultTextures};



pub struct Mesh {
    pub vao: VertexArray,
    pub vbo: Buffer,
    pub ebo: Option<Buffer>,

    pub index_count: i32,

    pub albedo: Texture2D,
    pub normal: Texture2D,
    pub orm: Texture2D,
}

impl Mesh {
    pub fn new(
        vertices: &[f32],
        indices: Option<&[u32]>,
        albedo_path: Option<&str>,
        normal_path: Option<&str>,
        orm_path: Option<&str>,
        defaults: &DefaultTextures,
    ) -> Self {
        let vao = VertexArray::new();
        let vbo = Buffer::new();

        vbo.upload(vertices, gl::STATIC_DRAW);

        vao.set_vertex_buffer(0, &vbo, 0, (5 * std::mem::size_of::<f32>()) as i32);
        vao.enable_attribute(0, 3, 0, 0);
        vao.enable_attribute(1, 2, 0, (3 * std::mem::size_of::<f32>()) as usize);

        let (ebo, index_count) = if let Some(indices) = indices {
            let ebo = Buffer::new();
            ebo.upload(indices, gl::STATIC_DRAW);
            vao.set_element_buffer(&ebo);
            (Some(ebo), indices.len() as i32)
        } else {
            (None, vertices.len() as i32 / 5) // 5 is number of components per vertex, e.g., x, y, z, u, v
        };

        // texture setup, if paths are provided, load textures, else None
        let albedo = albedo_path
            .map(|path| Texture2D::from_file(path, TextureFormat::SrgbRGBA))
            .unwrap_or_else(|| defaults.white.clone());

        let normal = normal_path
            .map(|path| Texture2D::from_file(path, TextureFormat::LinearRGBA))
            .unwrap_or_else(|| defaults.normal.clone());

        let orm = orm_path
            .map(|path| Texture2D::from_file(path, TextureFormat::LinearRGBA))
            .unwrap_or_else(|| defaults.black.clone());

        Self {
            vao,
            vbo,
            ebo,
            index_count,
            albedo,
            normal,
            orm,
        }
    }


    pub fn triangle(
        albedo_path: Option<&str>,
        normal_path: Option<&str>,
        orm_path: Option<&str>,
    ) -> Self {
        let vertices: [f32; 15] = [
            // positions       // tex coords
             0.0,  0.5, 0.0,   0.5, 1.0,
            -0.5, -0.5, 0.0,   0.0, 0.0,
             0.5, -0.5, 0.0,   1.0, 0.0,
        ];

        Self::new(&vertices, None, albedo_path, normal_path, orm_path, &DefaultTextures::new())
    }

    pub fn draw(&self) {
        self.vao.bind();

        self.albedo.bind(0);
        self.normal.bind(1);
        self.orm.bind(2);

        if self.ebo.is_some() {
            unsafe {
                gl::DrawElements(
                    gl::TRIANGLES,
                    self.index_count,
                    gl::UNSIGNED_INT,
                    std::ptr::null(),
                );
            }
        } else {
            unsafe {
                gl::DrawArrays(gl::TRIANGLES, 0, self.index_count);
            }
        }
    }
}