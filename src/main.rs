//! # Minecraft Clone
//! 
//! A Minecraft clone written in Rust using wgpu for rendering.

// Application modules.
mod renderer;
mod monitoring;
mod debug;

use std::sync::Arc;
use winit::{
    event::{Event, WindowEvent, KeyEvent, ElementState},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    keyboard::{PhysicalKey, KeyCode},
};
use log::{info, warn, error};

use crate::renderer::Renderer;
use crate::monitoring::SystemMonitor;
use crate::debug::overlay::DebugOverlay;

/// The main entry point of the application.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the logger.
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("Starting Minecraft Clone in Rust");

    // Create the event loop and window.
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

    // Initialize the wgpu renderer.
    let mut renderer = Renderer::new(Arc::clone(&window)).await?;
    info!("Renderer initialized with wgpu");

    // Initialize the system monitor for debug statistics.
    let mut system_monitor = SystemMonitor::new();
    info!("System monitor initialized");

    // Initialize the debug overlay.
    let mut debug_overlay = DebugOverlay::new();
    info!("Debug overlay initialized");

    let mut last_render_time = std::time::Instant::now();
    
    // Start the event loop.
    event_loop.run(move |event, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                // Pass events to the GUI for processing.
                renderer.gui_manager.handle_event(&window, event);
                
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
                    WindowEvent::KeyboardInput {
                        event: KeyEvent {
                            physical_key: PhysicalKey::Code(KeyCode::F3),
                            state: ElementState::Pressed,
                            ..
                        },
                        ..
                    } => {
                        // Toggle the debug overlay with F3.
                        debug_overlay.toggle();
                    }
                    WindowEvent::RedrawRequested => {
                        let now = std::time::Instant::now();
                        let dt = now - last_render_time;
                        last_render_time = now;

                        // Update the system monitor.
                        system_monitor.update();

                        // Render the scene.
                        match renderer.render(&window, &debug_overlay, &system_monitor) {
                            Ok(_) => {
                                system_monitor.record_frame(dt);
                            }
                            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                // Recreate the swap chain if it's lost or outdated.
                                renderer.resize(renderer.size());
                            }
                            Err(wgpu::SurfaceError::OutOfMemory) => {
                                error!("Out of memory!");
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
                // Request a redraw on the next cycle.
                window.request_redraw();
            }
            _ => {}
        }
    })?;

    Ok(())
}
