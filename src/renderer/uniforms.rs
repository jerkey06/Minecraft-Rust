use crate::renderer::camera::Camera;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub view_proj: [[f32; 4]; 4],
}

impl Uniforms {
    pub fn new() -> Self {
        use cgmath::{Matrix4, SquareMatrix};
        Self {
            view_proj: Matrix4::identity().into(),
        }
    }

    pub fn update_from_camera(&mut self, camera: &Camera, rotation: f32) {
        use cgmath::{Matrix4, Rad};
        
        let view = camera.get_view_matrix();
        let proj = camera.get_projection_matrix();
        let rotation_matrix = Matrix4::from_angle_y(Rad(rotation));
        
        self.view_proj = (proj * view * rotation_matrix).into();
    }
}