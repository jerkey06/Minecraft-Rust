//! # Módulo de Monitoreo
//! 
//! Este módulo proporciona herramientas para monitorear el rendimiento de la aplicación y las estadísticas del sistema.
//! Combina un perfilador de fotogramas y un monitor de estadísticas del sistema en una única estructura `SystemMonitor`.

pub mod frame_profiler;
pub mod system_stats;

use frame_profiler::FrameProfiler;
use system_stats::SystemStats;
use std::time::Duration;

/// Agrega el perfilador de fotogramas y las estadísticas del sistema en una única estructura.
pub struct SystemMonitor {
    frame_profiler: FrameProfiler,
    system_stats: SystemStats,
}

impl SystemMonitor {
    /// Crea una nueva instancia de `SystemMonitor`.
    pub fn new() -> Self {
        Self {
            frame_profiler: FrameProfiler::new(120),
            system_stats: SystemStats::new(),
        }
    }

    /// Actualiza las estadísticas del sistema.
    pub fn update(&mut self) {
        self.system_stats.refresh();
    }

    /// Registra un nuevo fotograma en el perfilador.
    pub fn record_frame(&mut self, frame_time: Duration) {
        self.frame_profiler.record(frame_time);
    }

    /// Devuelve el tiempo medio de fotograma en milisegundos.
    pub fn get_avg_frame_time_ms(&self) -> f64 {
        self.frame_profiler.get_avg_frame_time_ms()
    }

    /// Devuelve la marca de la CPU.
    pub fn get_cpu_brand(&self) -> &str {
        self.system_stats.get_cpu_brand()
    }

    /// Devuelve el uso actual de la CPU como un porcentaje.
    pub fn get_cpu_usage(&self) -> f32 {
        self.system_stats.get_cpu_usage()
    }

    /// Devuelve el uso actual de la GPU como un porcentaje.
    pub fn get_gpu_usage(&self) -> f32 {
        self.system_stats.get_gpu_usage()
    }

    /// Devuelve el uso actual de la memoria como un porcentaje.
    pub fn get_memory_usage_percent(&self) -> f64 {
        self.system_stats.get_memory_usage_percent()
    }

    /// Devuelve la memoria utilizada por el proceso actual en megabytes.
    pub fn get_process_memory_mb(&self) -> f64 {
        self.system_stats.get_process_memory_mb()
    }
}
