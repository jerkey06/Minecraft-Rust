pub mod frame_profiler;
pub mod system_stats;

use crate::monitoring::frame_profiler::FrameProfiler;
use crate::monitoring::system_stats::SystemStats;

use std::time::{Duration, Instant};

pub struct SystemMonitor {
    system_stats: SystemStats,
    frame_profiler: FrameProfiler,
    last_log_time: Instant,
    log_interval: Duration,
}

impl SystemMonitor {
    pub fn new() -> Self {
        Self {
            system_stats: SystemStats::new(),
            frame_profiler: FrameProfiler::new(300),
            last_log_time: Instant::now(),
            log_interval: Duration::from_secs(2),
        }
    }

    pub fn update(&mut self) {
        self.system_stats.refresh();

        if self.last_log_time.elapsed() >= self.log_interval {
            self.system_stats.log();
            self.last_log_time = Instant::now();
        }
    }

    pub fn record_frame(&mut self, frame_time: Duration) {
        self.frame_profiler.record(frame_time);

        if self.frame_profiler.should_log() {
            self.frame_profiler.log();
        }
    }

    // Métodos públicos para HUD
    pub fn get_cpu_usage(&self) -> f32 {
        self.system_stats.get_cpu_usage()
    }

    pub fn get_memory_usage_percent(&self) -> f64 {
        self.system_stats.get_memory_usage_percent()
    }

    pub fn get_process_memory_mb(&self) -> f64 {
        self.system_stats.get_process_memory_mb()
    }

    pub fn get_avg_frame_time_ms(&self) -> f64 {
        self.frame_profiler.get_avg_frame_time_ms()
    }
}
