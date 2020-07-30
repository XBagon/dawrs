use crate::{
    generator::{AdsrGenerator, Generator},
    patch::Patch,
    PolySample, SampleTiming,
};

#[derive(Clone)]
pub struct BasicSynthesizer<G: Generator> {
    pub base_generator: G,
    pub adsr: AdsrGenerator,
    pub volume: f32,
}

impl<G: Generator> BasicSynthesizer<G> {
    pub fn new(base_generator: G, adsr: AdsrGenerator, volume: f32) -> Self {
        BasicSynthesizer {
            base_generator,
            adsr,
            volume,
        }
    }
}

impl<G: Generator> Patch for BasicSynthesizer<G> {
    fn next_sample(&mut self, sample_timing: &SampleTiming) -> PolySample {
        let mut poly_sample = self.base_generator.generate(&sample_timing);

        poly_sample *= self.volume;

        let adsr = self.adsr.generate(&sample_timing);
        poly_sample.apply(&adsr);

        poly_sample
    }
}

impl<G: Generator + Default> Default for BasicSynthesizer<G> {
    fn default() -> Self {
        Self {
            base_generator: G::default(),
            adsr: Default::default(),
            volume: 0.1,
        }
    }
}
