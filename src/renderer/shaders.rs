//! # Shaders
//! 
//! Este módulo proporciona funciones para crear los módulos de shader de `wgpu`
//! a partir de los archivos de shader de WGSL.

/// Crea el módulo de shader básico.
pub fn create_basic_shader(device: &wgpu::Device) -> wgpu::ShaderModule {
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Basic Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shaders/basic.wgsl").into()),
    })
}

/// Crea el módulo de shader con textura.
pub fn create_textured_shader(device: &wgpu::Device) -> wgpu::ShaderModule {
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Textured Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shaders/textured.wgsl").into()),
    })
}
