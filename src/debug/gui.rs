//! # Gestor de GUI
//! 
//! Este módulo encapsula la lógica para renderizar interfaces gráficas de usuario (GUI)
//! utilizando `egui`. Se encarga de la inicialización, el manejo de eventos y el renderizado
//! de la GUI sobre la escena principal.

use egui::{Context, PlatformOutput, ViewportId};
use egui_wgpu::Renderer as EguiRenderer;
use egui_winit::State;
use winit::event::WindowEvent;
use winit::window::Window;

use crate::monitoring::SystemMonitor;
use crate::debug::overlay::DebugOverlay;

/// Gestiona el estado y el renderizado de la GUI de `egui`.
pub struct GuiManager {
    pub ctx: Context,
    pub state: State,
    pub renderer: EguiRenderer,
}

impl GuiManager {
    /// Crea una nueva instancia de `GuiManager`.
    /// 
    /// Inicializa el contexto de `egui`, el estado de la ventana y el renderizador de `egui-wgpu`.
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

    /// Maneja los eventos de la ventana y los pasa a `egui`.
    pub fn handle_event(&mut self, window: &Window, window_event: &WindowEvent) {
        let _ = self.state.on_window_event(window, window_event);
    }

    /// Renderiza la GUI en la pantalla.
    /// 
    /// Dibuja la superposición de depuración y cualquier otra interfaz de usuario definida.
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
        // Obtener la entrada de `egui` y ejecutar la lógica de la UI.
        let raw_input = self.state.take_egui_input(window);
        let full_output = self.ctx.run(raw_input, |ctx| {
            debug_overlay.ui(ctx, system_monitor, gpu_name);
        });

        // Manejar la salida de la plataforma (por ejemplo, copiar al portapapeles).
        self.state.handle_platform_output(window, full_output.platform_output);

        // Teselar las formas de `egui` en triángulos.
        let tris = self
            .ctx
            .tessellate(full_output.shapes, full_output.pixels_per_point);

        // Actualizar las texturas de `egui`.
        for (id, image_delta) in &full_output.textures_delta.set {
            self.renderer
                .update_texture(device, queue, *id, image_delta);
        }

        // Actualizar los buffers de `egui`.
        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [window.inner_size().width, window.inner_size().height],
            pixels_per_point: window.scale_factor() as f32,
        };

        self.renderer
            .update_buffers(device, queue, encoder, &tris, &screen_descriptor);

        // Renderizar la GUI.
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
