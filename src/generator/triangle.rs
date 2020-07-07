use super::Generator;
use crate::patch::SampleTiming;

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
    fn generate(&mut self, sample_timing: &SampleTiming) -> Vec<f32> {
        let sample_clock = self.sample_clock_from_frequency(&sample_timing, self.frequency);
        vec![
            ((((sample_clock * self.frequency * 4.0) / sample_timing.sample_rate) % 4.0) - 2.0)
                .abs()
                - 1.0,
        ]
    }
}
