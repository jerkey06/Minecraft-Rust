//! # Geometry
//! 
//! This module defines the geometry of the objects that are rendered in the scene.

use crate::renderer::vertex::Vertex;
use wgpu::util::DeviceExt;

/// Represents a cube with its vertex and index buffers.
pub struct Cube {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}

impl Cube {
    /// Creates a new cube.
    pub fn new(device: &wgpu::Device) -> Self {
        let vertices = vec![
            // Front face
            Vertex::new([-0.5, -0.5,  0.5], [1.0, 0.0, 0.0]),
            Vertex::new([ 0.5, -0.5,  0.5], [0.0, 1.0, 0.0]),
            Vertex::new([ 0.5,  0.5,  0.5], [0.0, 0.0, 1.0]),
            Vertex::new([-0.5,  0.5,  0.5], [1.0, 1.0, 0.0]),
            
            // Back face
            Vertex::new([-0.5, -0.5, -0.5], [1.0, 0.0, 1.0]),
            Vertex::new([ 0.5, -0.5, -0.5], [0.0, 1.0, 1.0]),
            Vertex::new([ 0.5,  0.5, -0.5], [0.5, 0.5, 0.5]),
            Vertex::new([-0.5,  0.5, -0.5], [1.0, 0.5, 0.0]),
        ];

        let indices: Vec<u16> = vec![
            // Front face
            0, 1, 2, 2, 3, 0,
            // Back face
            4, 6, 5, 6, 4, 7,
            // Left face
            4, 0, 3, 3, 7, 4,
            // Right face
            1, 5, 6, 6, 2, 1,
            // Top face
            3, 2, 6, 6, 7, 3,
            // Bottom face
            4, 5, 1, 1, 0, 4,
        ];
        
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Cube Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Cube Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        let num_indices = indices.len() as u32;
        
        Self {
            vertex_buffer,
            index_buffer,
            num_indices,
        }
    }
    
    /// Renders the cube.
    pub fn render<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>, 
        pipeline: &'a wgpu::RenderPipeline, 
        bind_group: &'a wgpu::BindGroup
    ) {
        render_pass.set_pipeline(pipeline);
        render_pass.set_bind_group(0, bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}