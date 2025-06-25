pub mod frame_profiler;
pub mod system_stats;

use frame_profiler::FrameProfiler;
use system_stats::SystemStats;
use std::time::Duration;

pub struct SystemMonitor {
    frame_profiler: FrameProfiler,
    system_stats: SystemStats,
}

impl SystemMonitor {
    pub fn new() -> Self {
        Self {
            frame_profiler: FrameProfiler::new(120),
            system_stats: SystemStats::new(),
        }
    }

    pub fn update(&mut self) {
        self.system_stats.refresh();
    }

    pub fn record_frame(&mut self, frame_time: Duration) {
        self.frame_profiler.record(frame_time);
    }

    pub fn get_avg_frame_time_ms(&self) -> f64 {
        self.frame_profiler.get_avg_frame_time_ms()
    }

    pub fn get_cpu_brand(&self) -> &str {
        self.system_stats.get_cpu_brand()
    }

    pub fn get_cpu_usage(&self) -> f32 {
        self.system_stats.get_cpu_usage()
    }

    pub fn get_gpu_usage(&self) -> f32 {
        self.system_stats.get_gpu_usage()
    }

    pub fn get_memory_usage_percent(&self) -> f64 {
        self.system_stats.get_memory_usage_percent()
    }

    pub fn get_process_memory_mb(&self) -> f64 {
        self.system_stats.get_process_memory_mb()
    }
}