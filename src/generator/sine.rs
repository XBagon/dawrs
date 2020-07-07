use super::Generator;
use crate::patch::SampleTiming;

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
    fn generate(&mut self, sample_timing: &SampleTiming) -> Vec<f32> {
        let sample_clock = self.sample_clock_from_frequency(&sample_timing, self.frequency);
        vec![(sample_clock * self.frequency * 2.0 * std::f32::consts::PI
            / sample_timing.sample_rate)
            .sin()]
    }
}
