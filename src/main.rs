//! # Minecraft Clone
//! 
//! Este es un clon de Minecraft escrito en Rust utilizando wgpu para el renderizado.
//! El proyecto sirve como un campo de pruebas para aprender sobre gráficos por computadora,
//! desarrollo de juegos y el ecosistema de Rust.

// Módulos principales de la aplicación.
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

/// Punto de entrada principal de la aplicación.
/// 
/// Inicializa el logger, la ventana, el renderizador y los sistemas de monitoreo.
/// Luego, entra en el bucle de eventos principal para manejar las interacciones del usuario
/// y renderizar la escena.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Inicializar el logger para mostrar mensajes en la consola.
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("Iniciando Minecraft Clone en Rust");

    // Crear el bucle de eventos y la ventana.
    let event_loop = EventLoop::new()?;
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Minecraft Clone - Rust")
            .with_inner_size(winit::dpi::LogicalSize::new(1280, 720))
            .with_min_inner_size(winit::dpi::LogicalSize::new(800, 600))
            .build(&event_loop)?
    );

    info!("Ventana creada: {}x{}", 
          window.inner_size().width, 
          window.inner_size().height);

    // Inicializar el renderizador de wgpu.
    let mut renderer = Renderer::new(Arc::clone(&window)).await?;
    info!("Renderizador inicializado con wgpu");

    // Inicializar el monitor del sistema para las estadísticas de depuración.
    let mut system_monitor = SystemMonitor::new();
    info!("Monitor del sistema inicializado");

    // Inicializar la superposición de depuración.
    let mut debug_overlay = DebugOverlay::new();
    info!("Superposición de depuración inicializada");

    let mut last_render_time = std::time::Instant::now();
    
    // Iniciar el bucle de eventos.
    event_loop.run(move |event, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                // Pasar eventos a la GUI para que los procese.
                renderer.gui_manager.handle_event(&window, event);
                
                match event {
                    WindowEvent::CloseRequested => {
                        info!("Cerrando la aplicación");
                        control_flow.exit();
                    }
                    WindowEvent::Resized(physical_size) => {
                        info!("Redimensionando la ventana: {}x{}", 
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
                        // Activar/desactivar la superposición de depuración con F3.
                        debug_overlay.toggle();
                    }
                    WindowEvent::RedrawRequested => {
                        let now = std::time::Instant::now();
                        let dt = now - last_render_time;
                        last_render_time = now;

                        // Actualizar el monitor del sistema.
                        system_monitor.update();

                        // Renderizar la escena.
                        match renderer.render(&window, &debug_overlay, &system_monitor) {
                            Ok(_) => {
                                system_monitor.record_frame(dt);
                            }
                            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                // Recrear la cadena de intercambio si se pierde o está desactualizada.
                                renderer.resize(renderer.size());
                            }
                            Err(wgpu::SurfaceError::OutOfMemory) => {
                                error!("¡Sin memoria!");
                                control_flow.exit();
                            }
                            Err(e) => {
                                warn!("Error de renderizado: {:?}", e);
                            }
                        }
                    }
                    _ => {}
                }
            }
            Event::AboutToWait => {
                // Solicitar un redibujado en el siguiente ciclo.
                window.request_redraw();
            }
            _ => {}
        }
    })?;

    Ok(())
}