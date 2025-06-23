pub fn create_basic_shader(device: &wgpu::Device) -> wgpu::ShaderModule {
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Basic Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shaders/basic.wgsl").into()),
    })
}

pub fn create_textured_shader(device: &wgpu::Device) -> wgpu::ShaderModule {
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Textured Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shaders/textured.wgsl").into()),
    })
}