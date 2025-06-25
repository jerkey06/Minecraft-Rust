//! # Superposición de Depuración
//! 
//! Este módulo define la superposición de depuración que se muestra al pulsar F3.
//! Muestra información útil como los FPS, el uso de la CPU y la memoria.

use egui::{Context, Window};
use crate::monitoring::SystemMonitor;

/// Contiene el estado de la superposición de depuración (por ejemplo, si está visible).
pub struct DebugOverlay {
    pub shown: bool,
}

impl Default for DebugOverlay {
    fn default() -> Self {
        Self::new()
    }
}

impl DebugOverlay {
    /// Crea una nueva instancia de `DebugOverlay`.
    pub fn new() -> Self {
        Self { shown: false }
    }

    /// Cambia la visibilidad de la superposición.
    pub fn toggle(&mut self) {
        self.shown = !self.shown;
    }

    /// Dibuja la interfaz de usuario de la superposición de depuración.
    pub fn ui(&self, ctx: &Context, monitor: &SystemMonitor, gpu_name: &str) {
        if !self.shown {
            return;
        }

        Window::new("Debug Info (F3)")
            .default_size([430.0, 140.0])
            .show(ctx, |ui| {
                ui.label(format!("FPS: {:.1}", 1.0 / monitor.get_avg_frame_time_ms() * 1000.0));
                ui.label(format!("CPU ({}): {:.1}%", monitor.get_cpu_brand(), monitor.get_cpu_usage()));
                ui.label(format!("GPU ({}): {:.1}%", gpu_name, monitor.get_gpu_usage()));
                ui.label(format!("RAM: {:.1}%", monitor.get_memory_usage_percent()));
                ui.label(format!("Proceso: {:.1} MB", monitor.get_process_memory_mb()));
            });
    }
}