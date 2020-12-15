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
    new_note: bool,
    pub muted: bool,
}

impl<G: Generator> BasicSynthesizer<G> {
    pub fn new(base_generator: G, adsr: AdsrGenerator, volume: f32) -> Self {
        BasicSynthesizer {
            base_generator,
            adsr,
            volume,
            new_note: false,
            muted: true,
        }
    }

    pub fn play(&mut self, sustain: f32) {
        self.adsr.sustain = sustain;
        self.new_note = true;
    }
}

impl<G: Generator> Patch for BasicSynthesizer<G> {
    fn next_sample(&mut self, sample_timing: &SampleTiming) -> PolySample {
        if self.new_note {
            self.new_note = false;
            self.adsr.start_tick = sample_timing.clock;
            self.muted = false;
        }

        if self.muted {
            PolySample(Vec::new())
        } else {
            let mut poly_sample = self.base_generator.generate(&sample_timing);

            poly_sample *= self.volume;

            let adsr = self.adsr.generate(&sample_timing);
            poly_sample.apply(&adsr);

            poly_sample
        }
    }
}

impl<G: Generator + Default> Default for BasicSynthesizer<G> {
    fn default() -> Self {
        Self {
            base_generator: G::default(),
            adsr: Default::default(),
            volume: 0.1,
            new_note: false,
            muted: true,
        }
    }
}
