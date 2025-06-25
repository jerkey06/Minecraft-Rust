use sysinfo::{System, CpuRefreshKind};

pub struct SystemStats {
    system: System,
    cpu_brand: String,
}

impl SystemStats {
    pub fn new() -> Self {
        let mut system = System::new();
        system.refresh_all();
        let cpu_brand = system.cpus().first().map_or("Unknown".to_string(), |cpu| cpu.brand().to_string());
        Self { system, cpu_brand }
    }

    pub fn refresh(&mut self) {
        self.system.refresh_cpu_specifics(CpuRefreshKind::everything());
        self.system.refresh_memory();
    }

    pub fn get_cpu_brand(&self) -> &str {
        &self.cpu_brand
    }

    pub fn get_cpu_usage(&self) -> f32 {
        self.system.global_cpu_usage()
    }

    pub fn get_gpu_usage(&self) -> f32 {
        // Nota: sysinfo no proporciona una forma directa de obtener el uso de la GPU.
        // Esta es una métrica difícil de obtener de forma multiplataforma.
        // Devolveremos 0.0 por ahora.
        0.0
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
