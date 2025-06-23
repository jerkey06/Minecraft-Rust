use std::time::{Duration, Instant};
use sysinfo::{System, Cpu, Process};
use log::info;

pub struct SystemMonitor {
    system: System,
    fps_counter: fps_counter::FPSCounter,
    last_log_time: Instant,
    log_interval: Duration,
    frame_times: Vec<Duration>,
    max_frame_samples: usize,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        
        Self {
            system,
            fps_counter: fps_counter::FPSCounter::new(),
            last_log_time: Instant::now(),
            log_interval: Duration::from_secs(2), // Log cada 2 segundos
            frame_times: Vec::new(),
            max_frame_samples: 300, // ~5 segundos a 60 FPS
        }
    }

    pub fn update(&mut self) {
        self.system.refresh_all();
        self.system.refresh_memory();
        self.system.refresh_processes();

        // Log periódico del estado del sistema
        if self.last_log_time.elapsed() >= self.log_interval {
            self.log_system_stats();
            self.last_log_time = Instant::now();
        }
    }

    pub fn record_frame(&mut self, frame_time: Duration) {
        // Actualizar contador de FPS
        let fps = self.fps_counter.tick();
        
        // Mantener historial de tiempos de frame
        self.frame_times.push(frame_time);
        if self.frame_times.len() > self.max_frame_samples {
            self.frame_times.remove(0);
        }

        // Log ocasional de performance de frames
        if self.frame_times.len() % 120 == 0 { // Cada ~2 segundos
            self.log_frame_stats(fps);
        }
    }

    fn log_system_stats(&mut self) {
        let total_memory = self.system.total_memory();
        let used_memory = self.system.used_memory();
        let memory_usage_percent = (used_memory as f64 / total_memory as f64) * 100.0;
        
        // CPU usage (promedio de todos los cores)
        let cpu_usage: f32 = self.system.cpus().iter()
            .map(|cpu| cpu.cpu_usage())
            .sum::<f32>() / self.system.cpus().len() as f32;

        // Información del proceso actual
        let current_pid = sysinfo::get_current_pid().unwrap();
        let process_memory = self.system.process(current_pid)
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

    fn log_frame_stats(&self, current_fps: usize) {
        if self.frame_times.is_empty() {
            return;
        }

        // Calcular estadísticas de frame time
        let total_time: Duration = self.frame_times.iter().sum();
        let avg_frame_time = total_time / self.frame_times.len() as u32;
        
        let mut sorted_times = self.frame_times.clone();
        sorted_times.sort();
        
        let min_frame_time = sorted_times[0];
        let max_frame_time = sorted_times[sorted_times.len() - 1];
        
        // Percentiles
        let p95_index = (sorted_times.len() as f32 * 0.95) as usize;
        let p99_index = (sorted_times.len() as f32 * 0.99) as usize;
        let p95_frame_time = sorted_times[p95_index.min(sorted_times.len() - 1)];
        let p99_frame_time = sorted_times[p99_index.min(sorted_times.len() - 1)];

        // Calcular frame drops (frames > 20ms = bajo de 50 FPS)
        let frame_drops = self.frame_times.iter()
            .filter(|&&t| t > Duration::from_millis(20))
            .count();
        let frame_drop_percent = (frame_drops as f64 / self.frame_times.len() as f64) * 100.0;

        info!(
            "🎮 Render | FPS: {} | Frame Time: avg={:.2}ms min={:.2}ms max={:.2}ms p95={:.2}ms p99={:.2}ms | Drops: {:.1}%",
            current_fps,
            avg_frame_time.as_secs_f64() * 1000.0,
            min_frame_time.as_secs_f64() * 1000.0,
            max_frame_time.as_secs_f64() * 1000.0,
            p95_frame_time.as_secs_f64() * 1000.0,
            p99_frame_time.as_secs_f64() * 1000.0,
            frame_drop_percent
        );
    }

    // Métodos públicos para obtener métricas en tiempo real
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
        if let Ok(current_pid) = sysinfo::get_current_pid() {
            self.system.process(current_pid)
                .map(|p| p.memory() as f64 / 1024.0 / 1024.0)
                .unwrap_or(0.0)
        } else {
            0.0
        }
    }

    pub fn get_avg_frame_time_ms(&self) -> f64 {
        if self.frame_times.is_empty() {
            return 0.0;
        }
        
        let total: Duration = self.frame_times.iter().sum();
        let avg = total / self.frame_times.len() as u32;
        avg.as_secs_f64() * 1000.0
    }
}