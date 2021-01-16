use super::Generator;
use crate::prelude::*;

#[derive(Clone)]
pub struct SineGenerator {
    pub frequency: f32,
}

impl SineGenerator {
    pub fn new(frequency: f32) -> Self {
        Self {
            frequency,
        }
    }
}

impl Default for SineGenerator {
    fn default() -> Self {
        Self::new(440.0)
    }
}

impl Generator for SineGenerator {
    fn generate(&mut self, sample_timing: &SampleTiming) -> PolySample {
        let sample_clock = sample_timing.sample_clock_with_frequency(self.frequency);
        poly_sample!([(sample_clock * self.frequency * 2.0 * std::f32::consts::PI).sin()])
    }
}
