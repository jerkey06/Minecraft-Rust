//! # System Stats
//! 
//! This module provides a `SystemStats` struct for getting information about
//! the system, such as CPU usage, memory, and process information.

use sysinfo::{System, CpuRefreshKind};

/// Collects and provides statistics about the system.
pub struct SystemStats {
    system: System,
    cpu_brand: String,
}

impl SystemStats {
    /// Creates a new `SystemStats`.
    pub fn new() -> Self {
        let mut system = System::new();
        system.refresh_all();
        let cpu_brand = system.cpus().first().map_or("Unknown".to_string(), |cpu| cpu.brand().to_string());
        Self { system, cpu_brand }
    }

    /// Refreshes the system statistics.
    pub fn refresh(&mut self) {
        self.system.refresh_cpu_specifics(CpuRefreshKind::everything());
        self.system.refresh_memory();
    }

    /// Returns the CPU brand.
    pub fn get_cpu_brand(&self) -> &str {
        &self.cpu_brand
    }

    /// Returns the current CPU usage as a percentage.
    pub fn get_cpu_usage(&self) -> f32 {
        self.system.global_cpu_usage()
    }

    /// Returns the current GPU usage as a percentage.
    /// 
    /// **Note:** `sysinfo` does not provide a reliable way to get GPU usage
    /// across different platforms. This currently returns `0.0`.
    pub fn get_gpu_usage(&self) -> f32 {
        0.0
    }

    /// Returns the current memory usage as a percentage.
    pub fn get_memory_usage_percent(&self) -> f64 {
        let total = self.system.total_memory();
        let used = self.system.used_memory();
        (used as f64 / total as f64) * 100.0
    }

    /// Returns the memory used by the current process in megabytes.
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
