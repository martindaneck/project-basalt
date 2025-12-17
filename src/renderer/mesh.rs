use super::{Buffer, VertexArray};

pub struct Mesh {
    pub vao: VertexArray,
    pub vbo: Buffer,
    pub vertex_count: i32,
}

impl Mesh {
    pub fn triangle() -> Self {
        let vertices: [f32; 9] = [
            0.0, 0.5, 0.0,   // Top vertex
            -0.5, -0.5, 0.0, // Bottom left vertex
            0.5, -0.5, 0.0,  // Bottom right vertex
        ];

        let vao = VertexArray::new();
        let vbo = Buffer::new();
        
        vbo.upload(&vertices, gl::STATIC_DRAW);
        
        vao.set_vertex_buffer(0, &vbo, 0, (3 * std::mem::size_of::<f32>()) as i32);
        vao.enable_attribute(0, 3, 0, 0);

        Self {
            vao,
            vbo,
            vertex_count: 3,
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao.id);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count);
        }
    }
}