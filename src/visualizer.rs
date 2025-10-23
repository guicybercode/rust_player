use rustfft::{num_complex::Complex, FftPlanner};
use std::collections::VecDeque;

pub struct Visualizer {
    fft_planner: FftPlanner<f32>,
    fft_size: usize,
    window: Vec<f32>,
    sample_buffer: VecDeque<f32>,
    spectrum_bars: Vec<f32>,
    beat_intensity: f32,
    rainbow_hue: f32,
    last_beat_time: std::time::Instant,
}

impl Visualizer {
    pub fn new() -> Self {
        let mut planner = FftPlanner::new();
        let fft_size = 2048;
        let _fft = planner.plan_fft_forward(fft_size);

        // Create Hann window
        let window: Vec<f32> = (0..fft_size)
            .map(|i| {
                0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / (fft_size - 1) as f32).cos())
            })
            .collect();

        Self {
            fft_planner: planner,
            fft_size,
            window,
            sample_buffer: VecDeque::with_capacity(fft_size * 2),
            spectrum_bars: vec![0.0; 32], // 32 frequency bars
            beat_intensity: 0.0,
            rainbow_hue: 0.0,
            last_beat_time: std::time::Instant::now(),
        }
    }

    pub fn add_samples(&mut self, samples: &[f32]) {
        for &sample in samples {
            self.sample_buffer.push_back(sample);
            if self.sample_buffer.len() > self.fft_size {
                self.sample_buffer.pop_front();
            }
        }
    }

    pub fn update_spectrum(&mut self) {
        if self.sample_buffer.len() < self.fft_size {
            return;
        }

        // Extract samples for FFT
        let mut samples: Vec<f32> = self.sample_buffer.iter().take(self.fft_size).cloned().collect();

        // Apply window function
        for (i, sample) in samples.iter_mut().enumerate() {
            *sample *= self.window[i];
        }

        // Convert to complex numbers
        let mut complex_samples: Vec<Complex<f32>> = samples
            .iter()
            .map(|&s| Complex::new(s, 0.0))
            .collect();

        // Perform FFT
        let fft = self.fft_planner.plan_fft_forward(self.fft_size);
        fft.process(&mut complex_samples);

        // Calculate magnitude spectrum
        let mut magnitudes: Vec<f32> = complex_samples
            .iter()
            .take(self.fft_size / 2)
            .map(|c| c.norm())
            .collect();

        // Normalize
        let max_magnitude = magnitudes.iter().fold(0.0_f32, |a, &b| a.max(b));
        if max_magnitude > 0.0 {
            for mag in &mut magnitudes {
                *mag /= max_magnitude;
            }
        }

        // Map to frequency bars
        self.update_frequency_bars(&magnitudes);

        // Detect beat
        self.detect_beat(&magnitudes);

        // Update rainbow hue
        self.update_rainbow_hue();
    }

    fn update_frequency_bars(&mut self, magnitudes: &[f32]) {
        let bar_count = self.spectrum_bars.len();
        let bin_per_bar = magnitudes.len() / bar_count;

        for (i, bar) in self.spectrum_bars.iter_mut().enumerate() {
            let start_bin = i * bin_per_bar;
            let end_bin = ((i + 1) * bin_per_bar).min(magnitudes.len());

            let avg_magnitude = if start_bin < end_bin {
                magnitudes[start_bin..end_bin].iter().sum::<f32>() / (end_bin - start_bin) as f32
            } else {
                0.0
            };

            // Smooth the bars
            *bar = *bar * 0.7 + avg_magnitude * 0.3;
        }
    }

    fn detect_beat(&mut self, magnitudes: &[f32]) {
        // Focus on low frequencies for beat detection (bass)
        let bass_range = 0..(magnitudes.len() / 8);
        let bass_energy: f32 = bass_range
            .map(|i| magnitudes[i])
            .sum::<f32>()
            .sqrt();

        // Simple beat detection: energy spike
        let threshold = 0.3;
        let now = std::time::Instant::now();
        let time_since_last_beat = now.duration_since(self.last_beat_time).as_secs_f32();

        if bass_energy > threshold && time_since_last_beat > 0.2 {
            self.beat_intensity = (bass_energy - threshold).min(1.0);
            self.last_beat_time = now;
        } else {
            // Decay beat intensity
            self.beat_intensity *= 0.95;
        }
    }

    fn update_rainbow_hue(&mut self) {
        // Rotate hue based on beat intensity
        let rotation_speed = 0.02 + self.beat_intensity * 0.1;
        self.rainbow_hue = (self.rainbow_hue + rotation_speed) % 360.0;
    }

    pub fn get_spectrum_bars(&self) -> &[f32] {
        &self.spectrum_bars
    }

    pub fn get_beat_intensity(&self) -> f32 {
        self.beat_intensity
    }

    pub fn get_rainbow_hue(&self) -> f32 {
        self.rainbow_hue
    }

    pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;

        let (r, g, b) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        (
            ((r + m) * 255.0) as u8,
            ((g + m) * 255.0) as u8,
            ((b + m) * 255.0) as u8,
        )
    }
}