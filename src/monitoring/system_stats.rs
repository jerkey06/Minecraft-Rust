use sysinfo::{System, get_current_pid};
use log::info;

pub struct SystemStats {
    system: System,
}

impl SystemStats {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        Self { system }
    }

    pub fn refresh(&mut self) {
        self.system.refresh_all();
    }

    pub fn log(&self) {
        let total_memory = self.system.total_memory();
        let used_memory = self.system.used_memory();
        let memory_usage_percent = (used_memory as f64 / total_memory as f64) * 100.0;

        let cpu_usage: f32 = self.system.cpus().iter()
            .map(|cpu| cpu.cpu_usage())
            .sum::<f32>() / self.system.cpus().len() as f32;

        let process_memory = self.system.process(sysinfo::get_current_pid().unwrap())
            .map(|p| p.memory())
            .unwrap_or(0);

        info!(
            "📊 Sistema | CPU: {:.1}% | RAM: {:.1}% ({}/{} MB) | Proceso: {:.1} MB",
            cpu_usage,
            memory_usage_percent,
            used_memory / 1024 / 1024,
            total_memory / 1024 / 1024,
            process_memory as f64 / 1024.0 / 1024.0
        );
    }

    pub fn get_cpu_usage(&self) -> f32 {
        self.system.cpus().iter()
            .map(|cpu| cpu.cpu_usage())
            .sum::<f32>() / self.system.cpus().len() as f32
    }

    pub fn get_memory_usage_percent(&self) -> f64 {
        let total = self.system.total_memory();
        let used = self.system.used_memory();
        (used as f64 / total as f64) * 100.0
    }

    pub fn get_process_memory_mb(&self) -> f64 {
        if let Ok(pid) = sysinfo::get_current_pid() {
            self.system.process(pid)
                .map(|p| p.memory() as f64 / 1024.0 / 1024.0)
                .unwrap_or(0.0)
        } else {
            0.0
        }
    }
}