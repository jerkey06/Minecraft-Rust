//! # Uniforms
//! 
//! This module defines the `Uniforms` struct, which contains data that is passed
//! to shaders, such as the view-projection matrix.

use crate::renderer::camera::Camera;

/// Contains the uniform data that is passed to shaders.
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub view_proj: [[f32; 4]; 4],
}

impl Uniforms {
    /// Creates a new `Uniforms` with an identity matrix.
    pub fn new() -> Self {
        use cgmath::{Matrix4, SquareMatrix};
        Self {
            view_proj: Matrix4::identity().into(),
        }
    }

    /// Updates the view-projection matrix from the camera and a rotation.
    pub fn update_from_camera(&mut self, camera: &Camera, rotation: f32) {
        use cgmath::{Matrix4, Rad};
        
        let view = camera.get_view_matrix();
        let proj = camera.get_projection_matrix();
        let rotation_matrix = Matrix4::from_angle_y(Rad(rotation));
        
        self.view_proj = (proj * view * rotation_matrix).into();
    }
}