use super::{TextureCube, LightCube};

pub struct EnvironmentMap {
    cube: LightCube, // vertices for rendering purposes
    pub environment: TextureCube,
}

impl EnvironmentMap {
    pub fn new(hdr_path: &str) -> Self {
        let environment = TextureCube::from_hdr_equirectangular(hdr_path, 1024);
        let cube = LightCube::new();
        Self { environment, cube }
    }

    pub fn draw_skybox(&self) {
        // Bind the environment texture
        self.environment.bind();

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
}