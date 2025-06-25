use egui::{Context, PlatformOutput, ViewportId};
use egui_wgpu::Renderer as EguiRenderer;
use egui_winit::State;
use winit::event::WindowEvent;
use winit::window::Window;

use crate::monitoring::SystemMonitor;
use crate::debug::overlay::DebugOverlay;

pub struct GuiManager {
    pub ctx: Context,
    pub state: State,
    pub renderer: EguiRenderer,
}

impl GuiManager {
    pub fn new(window: &Window, device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> Self {
        let ctx = Context::default();
        let state = State::new(ctx.clone(), ViewportId::ROOT, &window, None, None);
        let renderer = EguiRenderer::new(device, surface_format, None, 1);

        Self {
            ctx,
            state,
            renderer,
        }
    }

    pub fn handle_event(&mut self, window: &Window, window_event: &WindowEvent) {
        let _ = self.state.on_window_event(window, window_event);
    }

    pub fn render(
        &mut self,
        window: &Window,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        system_monitor: &SystemMonitor,
        debug_overlay: &DebugOverlay,
        gpu_name: &str,
    ) {
        let raw_input = self.state.take_egui_input(window);
        let full_output = self.ctx.run(raw_input, |ctx| {
            debug_overlay.ui(ctx, system_monitor, gpu_name);
        });

        self.state.handle_platform_output(window, full_output.platform_output);

        let tris = self
            .ctx
            .tessellate(full_output.shapes, full_output.pixels_per_point);

        for (id, image_delta) in &full_output.textures_delta.set {
            self.renderer
                .update_texture(device, queue, *id, image_delta);
        }

        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [window.inner_size().width, window.inner_size().height],
            pixels_per_point: window.scale_factor() as f32,
        };

        self.renderer
            .update_buffers(device, queue, encoder, &tris, &screen_descriptor);

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Egui Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });

        self.renderer.render(&mut render_pass, &tris, &screen_descriptor);
    }
}