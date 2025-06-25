//! # Monitoring Module
//! 
//! This module provides tools for monitoring application performance and system statistics.

pub mod frame_profiler;
pub mod system_stats;

use frame_profiler::FrameProfiler;
use system_stats::SystemStats;
use std::time::Duration;

/// Aggregates the frame profiler and system statistics into a single struct.
pub struct SystemMonitor {
    frame_profiler: FrameProfiler,
    system_stats: SystemStats,
}

impl SystemMonitor {
    /// Creates a new `SystemMonitor`.
    pub fn new() -> Self {
        Self {
            frame_profiler: FrameProfiler::new(120),
            system_stats: SystemStats::new(),
        }
    }

    /// Updates the system statistics.
    pub fn update(&mut self) {
        self.system_stats.refresh();
    }

    /// Records a new frame in the profiler.
    pub fn record_frame(&mut self, frame_time: Duration) {
        self.frame_profiler.record(frame_time);
    }

    /// Returns the average frame time in milliseconds.
    pub fn get_avg_frame_time_ms(&self) -> f64 {
        self.frame_profiler.get_avg_frame_time_ms()
    }

    /// Returns the CPU brand.
    pub fn get_cpu_brand(&self) -> &str {
        self.system_stats.get_cpu_brand()
    }

    /// Returns the current CPU usage as a percentage.
    pub fn get_cpu_usage(&self) -> f32 {
        self.system_stats.get_cpu_usage()
    }

    /// Returns the current GPU usage as a percentage.
    pub fn get_gpu_usage(&self) -> f32 {
        self.system_stats.get_gpu_usage()
    }

    /// Returns the current memory usage as a percentage.
    pub fn get_memory_usage_percent(&self) -> f64 {
        self.system_stats.get_memory_usage_percent()
    }

    /// Returns the memory used by the current process in megabytes.
    pub fn get_process_memory_mb(&self) -> f64 {
        self.system_stats.get_process_memory_mb()
    }
}