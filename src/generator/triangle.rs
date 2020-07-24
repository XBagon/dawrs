use super::Generator;
use crate::SampleTiming;

#[derive(Clone)]
pub struct TriangleGenerator {
    pub frequency: f32,
    pub start_tick: usize,
}

impl TriangleGenerator {
    pub fn new(frequency: f32) -> Self {
        Self {
            frequency,
            start_tick: 0,
        }
    }
}

impl Default for TriangleGenerator {
    fn default() -> Self {
        Self::new(440.0)
    }
}

impl Generator for TriangleGenerator {
    fn generate(&mut self, sample_timing: &SampleTiming) -> Vec<f32> {
        let sample_clock =
            sample_timing.sample_clock_with_frequency(self.frequency, self.start_tick);
        vec![(((sample_clock * self.frequency * 4.0) % 4.0) - 2.0).abs() - 1.0]
    }
}
