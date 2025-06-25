//! # Estadísticas del Sistema
//! 
//! Este módulo proporciona una estructura `SystemStats` para obtener información sobre
//! el sistema, como el uso de la CPU, la memoria y la información del proceso.

use sysinfo::{System, CpuRefreshKind};

/// Recopila y proporciona estadísticas sobre el sistema.
pub struct SystemStats {
    system: System,
    cpu_brand: String,
}

impl SystemStats {
    /// Crea una nueva instancia de `SystemStats`.
    pub fn new() -> Self {
        let mut system = System::new();
        system.refresh_all();
        let cpu_brand = system.cpus().first().map_or("Unknown".to_string(), |cpu| cpu.brand().to_string());
        Self { system, cpu_brand }
    }

    /// Actualiza las estadísticas del sistema.
    pub fn refresh(&mut self) {
        self.system.refresh_cpu_specifics(CpuRefreshKind::everything());
        self.system.refresh_memory();
    }

    /// Devuelve la marca de la CPU.
    pub fn get_cpu_brand(&self) -> &str {
        &self.cpu_brand
    }

    /// Devuelve el uso actual de la CPU como un porcentaje.
    pub fn get_cpu_usage(&self) -> f32 {
        self.system.global_cpu_usage()
    }

    /// Devuelve el uso actual de la GPU como un porcentaje.
    /// 
    /// **Nota:** `sysinfo` no proporciona una forma fiable de obtener el uso de la GPU
    /// de forma multiplataforma. Actualmente, esto devuelve `0.0`.
    pub fn get_gpu_usage(&self) -> f32 {
        0.0
    }

    /// Devuelve el uso actual de la memoria como un porcentaje.
    pub fn get_memory_usage_percent(&self) -> f64 {
        let total = self.system.total_memory();
        let used = self.system.used_memory();
        (used as f64 / total as f64) * 100.0
    }

    /// Devuelve la memoria utilizada por el proceso actual en megabytes.
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