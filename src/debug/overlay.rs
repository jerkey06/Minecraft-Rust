use egui::{Context, Window, Label};
use crate::monitoring::SystemMonitor;

pub struct DebugOverlay {
    pub shown: bool,
}

impl Default for DebugOverlay {
    fn default() -> Self {
        Self::new()
    }
}

impl DebugOverlay {
    pub fn new() -> Self {
        Self { shown: false }
    }

    pub fn toggle(&mut self) {
        self.shown = !self.shown;
    }

    pub fn ui(&self, ctx: &Context, monitor: &SystemMonitor, gpu_name: &str) {
        if !self.shown {
            return;
        }

        Window::new("Debug Info (F3)")
            .default_size([220.0, 120.0])
            .show(ctx, |ui| {
                ui.label(format!("FPS: {:.1}", 1.0 / monitor.get_avg_frame_time_ms() * 1000.0));
                ui.label(format!("CPU ({}): {:.1}%", monitor.get_cpu_brand(), monitor.get_cpu_usage()));
                ui.label(format!("GPU ({}): {:.1}%", gpu_name, monitor.get_gpu_usage()));
                ui.label(format!("RAM: {:.1}%", monitor.get_memory_usage_percent()));
                ui.label(format!("Proceso: {:.1} MB", monitor.get_process_memory_mb()));
            });
    }
}
