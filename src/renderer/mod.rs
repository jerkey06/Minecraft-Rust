//! # Renderer Module
//! 
//! This module is the core of the rendering engine. It handles `wgpu` initialization,
//! manages the render surface, creates the render pipelines, and draws the scene
//! every frame.

mod vertex;
mod uniforms;
mod geometry;
mod camera;

pub use vertex::Vertex;
pub use uniforms::Uniforms;
pub use geometry::Cube;
pub use camera::Camera;

use std::sync::Arc;
use wgpu::util::DeviceExt;
use winit::window::Window;
use log::info;

use crate::debug::gui::GuiManager;
use crate::debug::overlay::DebugOverlay;
use crate::monitoring::SystemMonitor;

/// Manages all rendering-related aspects.
pub struct Renderer {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    
    // Adapter information
    pub gpu_name: String,

    // Separate components
    camera: Camera,
    cube: Cube,
    
    // Buffers and resources
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    
    // GUI
    pub gui_manager: GuiManager,

    // State
    rotation: f32,
}

impl Renderer {
    /// Creates a new `Renderer`.
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let size = window.inner_size();
        
        // Initialize wgpu
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });
        
        let surface = instance.create_surface(window.clone())?;
        
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.ok_or_else(|| anyhow::anyhow!("Failed to get adapter"))?;

        let gpu_name = adapter.get_info().name;
        info!("GPU: {:?}", gpu_name);
        info!("Backend: {:?}", adapter.get_info().backend);

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        ).await?;

        // Configure the surface
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
            
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        
        surface.configure(&device, &config);

        // Create the GUI
        let gui_manager = GuiManager::new(&window, &device, config.format);

        // Create the shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        // Create the camera
        let camera = Camera::new(
            cgmath::Point3::new(0.0, 0.0, 3.0),
            cgmath::Point3::new(0.0, 0.0, 0.0),
            cgmath::Vector3::unit_y(),
            45.0,
            size.width as f32 / size.height as f32,
            0.1,
            100.0,
        );

        // Create the uniforms
        let mut uniforms = Uniforms::new();
        uniforms.update_from_camera(&camera, 0.0);

        let uniform_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice(&[uniforms]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("uniform_bind_group_layout"),
        });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                }
            ],
            label: Some("uniform_bind_group"),
        });

        // Create the render pipeline
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        // Create the geometry
        let cube = Cube::new(&device);

        Ok(Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            gpu_name,
            camera,
            cube,
            uniform_buffer,
            uniform_bind_group,
            gui_manager,
            rotation: 0.0,
        })
    }

    /// Resizes the render surface when the window size changes.
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            
            // Update the camera's aspect ratio.
            self.camera.set_aspect_ratio(new_size.width as f32 / new_size.height as f32);
        }
    }

    /// Returns the current size of the render surface.
    pub fn size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.size
    }

    /// Renders a single frame.
    pub fn render(&mut self, window: &Window, debug_overlay: &DebugOverlay, system_monitor: &SystemMonitor) -> Result<(), wgpu::SurfaceError> {
        // Update the cube's rotation.
        self.rotation += 0.01;

        // Update the uniforms.
        let mut uniforms = Uniforms::new();
        uniforms.update_from_camera(&self.camera, self.rotation);

        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[uniforms]),
        );

        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            // Render the cube.
            self.cube.render(&mut render_pass, &self.render_pipeline, &self.uniform_bind_group);
        }

        // Render the GUI.
        self.gui_manager.render(window, &self.device, &self.queue, &mut encoder, &view, system_monitor, debug_overlay, &self.gpu_name);

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
