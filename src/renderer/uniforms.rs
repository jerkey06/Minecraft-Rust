//! # Uniforms
//! 
//! Este módulo define la estructura `Uniforms` que contiene los datos que se pasan
//! a los shaders, como la matriz de vista-proyección.

use crate::renderer::camera::Camera;

/// Contiene los datos uniformes que se pasan a los shaders.
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub view_proj: [[f32; 4]; 4],
}

impl Uniforms {
    /// Crea una nueva instancia de `Uniforms` con una matriz de identidad.
    pub fn new() -> Self {
        use cgmath::{Matrix4, SquareMatrix};
        Self {
            view_proj: Matrix4::identity().into(),
        }
    }

    /// Actualiza la matriz de vista-proyección a partir de la cámara y una rotación.
    pub fn update_from_camera(&mut self, camera: &Camera, rotation: f32) {
        use cgmath::{Matrix4, Rad};
        
        let view = camera.get_view_matrix();
        let proj = camera.get_projection_matrix();
        let rotation_matrix = Matrix4::from_angle_y(Rad(rotation));
        
        self.view_proj = (proj * view * rotation_matrix).into();
    }
}
