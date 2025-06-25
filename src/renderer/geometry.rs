//! # Geometría
//! 
//! Este módulo define la geometría de los objetos que se renderizan en la escena.
//! Actualmente, solo contiene una estructura `Cube` para crear y renderizar un cubo.

use crate::renderer::vertex::Vertex;
use wgpu::util::DeviceExt;

/// Representa un cubo con sus vértices y búferes de índices.
pub struct Cube {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}

impl Cube {
    /// Crea un nuevo cubo.
    /// 
    /// Crea los búferes de vértices e índices para un cubo y los sube a la GPU.
    pub fn new(device: &wgpu::Device) -> Self {
        let vertices = vec![
            // Cara frontal
            Vertex::new([-0.5, -0.5,  0.5], [1.0, 0.0, 0.0]),
            Vertex::new([ 0.5, -0.5,  0.5], [0.0, 1.0, 0.0]),
            Vertex::new([ 0.5,  0.5,  0.5], [0.0, 0.0, 1.0]),
            Vertex::new([-0.5,  0.5,  0.5], [1.0, 1.0, 0.0]),
            
            // Cara trasera
            Vertex::new([-0.5, -0.5, -0.5], [1.0, 0.0, 1.0]),
            Vertex::new([ 0.5, -0.5, -0.5], [0.0, 1.0, 1.0]),
            Vertex::new([ 0.5,  0.5, -0.5], [0.5, 0.5, 0.5]),
            Vertex::new([-0.5,  0.5, -0.5], [1.0, 0.5, 0.0]),
        ];

        let indices: Vec<u16> = vec![
            // Cara frontal
            0, 1, 2, 2, 3, 0,
            // Cara trasera
            4, 6, 5, 6, 4, 7,
            // Cara izquierda
            4, 0, 3, 3, 7, 4,
            // Cara derecha
            1, 5, 6, 6, 2, 1,
            // Cara superior
            3, 2, 6, 6, 7, 3,
            // Cara inferior
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
    
    /// Renderiza el cubo.
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
