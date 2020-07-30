use super::Generator;
use crate::{SampleTiming, PolySample};

#[derive(Clone)]
pub struct SineGenerator {
    pub frequency: f32,
    pub start_tick: usize,
}

impl SineGenerator {
    pub fn new(frequency: f32) -> Self {
        Self {
            frequency,
            start_tick: 0,
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
        let sample_clock =
            sample_timing.sample_clock_with_frequency(self.frequency, self.start_tick);
        PolySample(vec![(sample_clock * self.frequency * 2.0 * std::f32::consts::PI).sin()])
    }
}
