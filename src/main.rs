mod renderer;
mod monitoring;

use std::sync::Arc;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use log::{info, warn, error};

use crate::renderer::Renderer;
use crate::monitoring::SystemMonitor;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("Starting Minecraft Clone in Rust");

    let event_loop = EventLoop::new()?;
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Minecraft Clone - Rust")
            .with_inner_size(winit::dpi::LogicalSize::new(1280, 720))
            .with_min_inner_size(winit::dpi::LogicalSize::new(800, 600))
            .build(&event_loop)?
    );

    info!("Window created: {}x{}", 
          window.inner_size().width, 
          window.inner_size().height);

    let mut renderer = Renderer::new(Arc::clone(&window)).await?;
    info!("Renderer initialized with wgpu");

    let mut system_monitor = SystemMonitor::new();
    info!("System monitor initialized");

    let mut last_render_time = std::time::Instant::now();
    
    event_loop.run(move |event, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested => {
                        info!("Closing application");
                        control_flow.exit();
                    }
                    WindowEvent::Resized(physical_size) => {
                        info!("Resizing window: {}x{}", 
                              physical_size.width, physical_size.height);
                        renderer.resize(*physical_size);
                    }
                    WindowEvent::RedrawRequested => {
                        let now = std::time::Instant::now();
                        let dt = now - last_render_time;
                        last_render_time = now;

                        system_monitor.update();

                        match renderer.render() {
                            Ok(_) => {
                                system_monitor.record_frame(dt);
                            }
                            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                renderer.resize(renderer.size());
                            }
                            Err(wgpu::SurfaceError::OutOfMemory) => {
                                error!("Out of memory");
                                control_flow.exit();
                            }
                            Err(e) => {
                                warn!("Render error: {:?}", e);
                            }
                        }
                    }
                    _ => {}
                }
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {}
        }
    })?;

    Ok(())
}