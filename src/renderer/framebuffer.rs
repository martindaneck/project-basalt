use super::{Texture2D};

pub struct Framebuffer {
    id: u32,
    pub width: u32,
    pub height: u32,
    pub color: Vec<Texture2D>,
    pub depth: Option<Texture2D>,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let mut id = 0;
        unsafe {
            gl::CreateFramebuffers(1, &mut id);
        }

        Self {
            id,
            width,
            height,
            color: Vec::new(),
            depth: None,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    pub fn add_color_attachment(&mut self, texture: Texture2D) {
        let index = self.color.len() as u32;
        unsafe {
            gl::NamedFramebufferTexture(self.id, gl::COLOR_ATTACHMENT0 + index, texture.id, 0);
        }
        self.color.push(texture);

        let attachments: Vec<u32> = (0..self.color.len())
            .map(|i| gl::COLOR_ATTACHMENT0 + i as u32)
            .collect();
        unsafe {
            gl::NamedFramebufferDrawBuffers(
                self.id,
                attachments.len() as i32,
                attachments.as_ptr(),
            );
        }
    }

    pub fn set_depth_attachment(&mut self, texture: Texture2D) {
        unsafe {
            gl::NamedFramebufferTexture(self.id, gl::DEPTH_ATTACHMENT, texture.id, 0);
        }
        self.depth = Some(texture);
    }

    pub fn check_complete(&self) {
        let status = unsafe {
            gl::CheckFramebufferStatus(gl::FRAMEBUFFER) == gl::FRAMEBUFFER_COMPLETE
        };
        assert!(status, "Framebuffer is not complete!");
    }
}

