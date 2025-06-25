//! # Camera
//! 
//! This module defines a 3D perspective camera that can be moved and rotated.

use cgmath::{Matrix4, Point3, Vector3, perspective, Deg, InnerSpace, Rad, Matrix3};

/// Represents a camera in 3D space.
pub struct Camera {
    pub position: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
    pub fovy: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    /// Creates a new camera.
    pub fn new(
        position: Point3<f32>, 
        target: Point3<f32>, 
        up: Vector3<f32>,
        fovy_degrees: f32,
        aspect: f32,
        near: f32,
        far: f32,
    ) -> Self {
        Self {
            position,
            target,
            up,
            fovy: fovy_degrees,
            aspect,
            near,
            far,
        }
    }
    
    /// Calculates the camera's view matrix.
    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(self.position, self.target, self.up)
    }
    
    /// Calculates the camera's projection matrix.
    pub fn get_projection_matrix(&self) -> Matrix4<f32> {
        perspective(Deg(self.fovy), self.aspect, self.near, self.far)
    }
    
    /// Sets the camera's aspect ratio.
    pub fn set_aspect_ratio(&mut self, aspect: f32) {
        self.aspect = aspect;
    }
    
    /// Moves the camera forward or backward.
    pub fn move_forward(&mut self, distance: f32) {
        let forward = (self.target - self.position).normalize();
        self.position += forward * distance;
        self.target += forward * distance;
    }
    
    /// Moves the camera to the right or left.
    pub fn move_right(&mut self, distance: f32) {
        let forward = (self.target - self.position).normalize();
        let right = forward.cross(self.up).normalize();
        self.position += right * distance;
        self.target += right * distance;
    }
    
    /// Rotates the camera around its target.
    pub fn rotate_around_target(&mut self, yaw: f32, pitch: f32) {
        let offset: Vector3<f32> = self.position - self.target;
        let rotation: Matrix3<f32> = Matrix3::from_angle_y(Rad(yaw)) * Matrix3::from_angle_x(Rad(pitch));
        let rotated_offset = rotation * offset;

        self.position = self.target + rotated_offset;
    }
}