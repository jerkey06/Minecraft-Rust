use egui::{Context, Window, Label};
use crate::monitoring::SystemMonitor;

pub struct DebugOverlay;

impl DebugOverlay {
    pub fn show(ctx: &Context, monitor: &SystemMonitor) {
        Window::new("Debug Info (F3)")
            .default_size([220.0, 120.0])
            .show(ctx, |ui| {
                ui.label(Label::new(format!("FPS: {:.1}", 1000.0 / monitor.get_avg_frame_time_ms())));
                ui.label(Label::new(format!("CPU: {:.1}%", monitor.get_cpu_usage())));
                ui.label(Label::new(format!("RAM: {:.1}%", monitor.get_memory_usage_percent())));
                ui.label(Label::new(format!("Proceso: {:.1} MB", monitor.get_process_memory_mb())));
            });
    }
}