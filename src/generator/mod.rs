use crate::patch::SampleTiming;

pub trait Generator {
    fn generate(&mut self, patch_base: &SampleTiming) -> Vec<f32>;
}

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
    fn generate(&mut self, patch_base: &SampleTiming) -> Vec<f32> {
        vec![(patch_base.sample_clock * self.frequency * 2.0 * std::f32::consts::PI
            / patch_base.sample_rate)
            .sin()]
    }
}
