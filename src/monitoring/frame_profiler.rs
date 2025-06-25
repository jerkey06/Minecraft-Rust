//! # Perfilador de Fotogramas
//! 
//! Este módulo proporciona una estructura `FrameProfiler` para medir y registrar
//! el tiempo de renderizado de los fotogramas y calcular los FPS (fotogramas por segundo).

use std::time::Duration;
use log::info;

/// Mide y calcula estadísticas sobre los tiempos de fotograma.
pub struct FrameProfiler {
    frame_times: Vec<Duration>,
    max_samples: usize,
    frames_recorded: usize,
    current_fps: usize,
}

impl FrameProfiler {
    /// Crea una nueva instancia de `FrameProfiler`.
    /// 
    /// - `max_samples`: El número máximo de tiempos de fotograma a almacenar para el cálculo de la media.
    pub fn new(max_samples: usize) -> Self {
        Self {
            frame_times: Vec::with_capacity(max_samples),
            max_samples,
            frames_recorded: 0,
            current_fps: 0,
        }
    }

    /// Registra el tiempo que ha tardado en renderizarse un nuevo fotograma.
    pub fn record(&mut self, frame_time: Duration) {
        self.frame_times.push(frame_time);
        if self.frame_times.len() > self.max_samples {
            self.frame_times.remove(0);
        }

        self.frames_recorded += 1;
        self.current_fps = (1.0 / frame_time.as_secs_f64()).round() as usize;
    }

    /// Devuelve `true` si se debe registrar un nuevo conjunto de estadísticas.
    pub fn should_log(&self) -> bool {
        self.frames_recorded % 120 == 0
    }

    /// Registra las estadísticas de tiempo de fotograma en la consola.
    pub fn log(&self) {
        if self.frame_times.is_empty() {
            return;
        }

        let total: Duration = self.frame_times.iter().sum();
        let avg = total / self.frame_times.len() as u32;

        let mut sorted = self.frame_times.clone();
        sorted.sort();

        let min = sorted[0];
        let max = sorted[sorted.len() - 1];

        let p95 = sorted[((sorted.len() as f32) * 0.95) as usize];
        let p99 = sorted[((sorted.len() as f32) * 0.99) as usize];

        let drops = self.frame_times.iter()
            .filter(|&&t| t > Duration::from_millis(20))
            .count();
        let drop_pct = (drops as f64 / self.frame_times.len() as f64) * 100.0;

        info!(
            "🎮 Render | FPS: {} | Frame Time: avg={:.2}ms min={:.2}ms max={:.2}ms p95={:.2}ms p99={:.2}ms | Drops: {:.1}%",
            self.current_fps,
            avg.as_secs_f64() * 1000.0,
            min.as_secs_f64() * 1000.0,
            max.as_secs_f64() * 1000.0,
            p95.as_secs_f64() * 1000.0,
            p99.as_secs_f64() * 1000.0,
            drop_pct
        );
    }

    /// Devuelve el tiempo medio de fotograma en milisegundos.
    pub fn get_avg_frame_time_ms(&self) -> f64 {
        if self.frame_times.is_empty() {
            return 0.0;
        }

        let total: Duration = self.frame_times.iter().sum();
        let avg = total / self.frame_times.len() as u32;
        avg.as_secs_f64() * 1000.0
    }
}
