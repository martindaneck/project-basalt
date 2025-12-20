use imgui::*;
use glfw::Window;

use crate::renderer::buffer::Buffer;

/// this must exactly match the glsl std140 layout
#[repr(C)]
#[derive(Clone, Copy)]
struct GlobalUniforms {
    gamma: f32,
    exposure: f32,
    _padding: [f32; 2], // padding to make size multiple of 16 bytes
}

pub struct UI {
    // imgui context
    

}