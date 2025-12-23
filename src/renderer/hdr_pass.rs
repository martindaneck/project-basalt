use super::framebuffer::Framebuffer;
use super::Texture2D;

pub struct HdrPass {
    pub framebuffer: Framebuffer,
}

impl HdrPass {
    pub fn new(width: u32, height: u32) -> Self {
        let mut framebuffer = Framebuffer::new(width, height);
        let color = Texture2D::empty(width, height, gl::RGBA16F, gl::LINEAR, gl::REPEAT);
        let depth = Texture2D::empty(width, height, gl::DEPTH_COMPONENT24, gl::NEAREST, gl::CLAMP_TO_EDGE);
        framebuffer.add_color_attachment(color);
        framebuffer.set_depth_attachment(depth);
        framebuffer.check_complete();
        Self { framebuffer }
    }

    pub fn begin(&mut self) {
        self.framebuffer.bind();
        unsafe {
            gl::Viewport(0, 0, self.framebuffer.width as i32, self.framebuffer.height as i32);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
    
    pub fn end(&mut self) {
        self.framebuffer.unbind();
    }
}