use super::Generator;
use crate::prelude::*;

#[derive(Clone)]
pub struct TriangleGenerator {
    pub frequency: f32,
}

impl TriangleGenerator {
    pub fn new(frequency: f32) -> Self {
        Self {
            frequency,
        }
    }
}

impl Default for TriangleGenerator {
    fn default() -> Self {
        Self::new(440.0)
    }
}

impl Generator for TriangleGenerator {
    fn generate(&mut self, sample_timing: &SampleTiming) -> PolySample {
        let sample_clock = sample_timing.sample_clock_with_frequency(self.frequency);
        poly_sample!([(((sample_clock * self.frequency * 4.0) % 4.0) - 2.0).abs() - 1.0])
    }
}
