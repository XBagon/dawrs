use super::Generator;
use crate::{PolySample, SampleTiming};

#[derive(Clone)]
pub struct AdsrGenerator {
    pub attack: f32,
    pub decay: f32,
    pub sustain_level: f32,
    pub sustain: f32,
    pub release: f32,
    pub start_tick: usize,
}

impl AdsrGenerator {
    pub fn new(attack: f32, decay: f32, sustain_level: f32, sustain: f32, release: f32) -> Self {
        Self {
            attack,
            decay,
            sustain,
            sustain_level,
            release,
            start_tick: 0,
        }
    }
}

impl Default for AdsrGenerator {
    fn default() -> Self {
        Self::new(0.0, 0.0, 1.0, 0.0, 0.0)
    }
}

impl Generator for AdsrGenerator {
    fn generate(&mut self, sample_timing: &SampleTiming) -> PolySample {
        let mut sample_clock = sample_timing.sample_clock(self.start_tick);

        let value = if sample_clock < self.attack {
            sample_clock / self.attack
        } else {
            sample_clock -= self.attack;
            if sample_clock < self.decay {
                1.0 - ((sample_clock / self.decay) * (1.0 - self.sustain_level))
            } else {
                sample_clock -= self.decay;
                if sample_clock < self.sustain {
                    self.sustain_level
                } else {
                    sample_clock -= self.sustain;
                    if sample_clock < self.release {
                        (1.0 - ((sample_clock) / self.release)) * self.sustain_level
                    } else {
                        0.0
                    }
                }
            }
        };
        PolySample(vec![value])
    }
}
