use gltf::Camera;
use glam::{Mat4,Vec3};

use super::{Buffer};

// std140 structs
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SettingsUniforms {
    gamma: f32,
    exposure: f32,
    rendermode: u32,
    _padding: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CameraUniforms {
    view_matrix: [[f32; 4]; 4],
    projection_matrix: [[f32; 4]; 4],
    camera_position: [f32; 3],
    _padding: f32, // padding to make size multiple of 16 bytes
}


pub struct UboManager {
    settings: SettingsUniforms,
    settings_ubo: Buffer,
    settings_dirty: bool,

    camera: CameraUniforms,
    camera_ubo: Buffer,
    camera_dirty: bool, 
}

impl UboManager {
    pub fn new() -> Self {
        let settings = SettingsUniforms {
            gamma: 2.2,
            exposure: 1.0,
            rendermode: 1,
            _padding: 0.0,
        };
        let settings_ubo = Buffer::new();
        settings_ubo.allocate::<SettingsUniforms>(std::mem::size_of::<SettingsUniforms>(), gl::DYNAMIC_DRAW);
        settings_ubo.write(&[settings]);

        let camera = CameraUniforms {
            view_matrix: [[0.0; 4]; 4],
            projection_matrix: [[0.0; 4]; 4],
            camera_position: [0.0; 3],
            _padding: 0.0,
        };
        let camera_ubo = Buffer::new();
        camera_ubo.allocate::<CameraUniforms>(std::mem::size_of::<CameraUniforms>(), gl::DYNAMIC_DRAW);
        camera_ubo.write(&[camera]);

        // bind settings and camera UBOs
        settings_ubo.bind_base(0); // Binding point 0 for settings
        camera_ubo.bind_base(1); // Binding point 1 for camera

        Self {
            settings,
            settings_ubo,
            settings_dirty: false,
            camera,
            camera_ubo,
            camera_dirty: false,
        }
    }
    // Settings
    // this is a prototype, no input yet
    pub fn set_settings(&mut self) -> &mut SettingsUniforms {
        self.settings_dirty = true;
        &mut self.settings
    }

    // Camera
    pub fn set_camera(
        &mut self, 
        (view_matrix, projection_matrix, camera_position): (
            Mat4,
            Mat4,
            Vec3
        ),
    ) {
        self.camera.view_matrix = view_matrix.to_cols_array_2d();
        self.camera.projection_matrix = projection_matrix.to_cols_array_2d();
        self.camera.camera_position = camera_position.to_array();
        self.camera_dirty = true;
    }

    // Update UBOs if dirty
    pub fn update(&mut self) {
        if self.settings_dirty {
            self.settings_ubo.write(&[self.settings]);
            self.settings_dirty = false;
        }
        if self.camera_dirty {
            self.camera_ubo.write(&[self.camera]);
            self.camera_dirty = false;
        }
    }
}