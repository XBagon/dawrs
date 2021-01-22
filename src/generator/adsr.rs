use super::Generator;
use crate::prelude::*;

#[derive(Clone)]
pub struct AdsrGenerator {
    pub attack: f32,
    pub decay: f32,
    pub sustain_level: f32,
    pub sustain: f32,
    pub release: f32,
}

impl AdsrGenerator {
    pub fn new(attack: f32, decay: f32, sustain_level: f32, sustain: f32, release: f32) -> Self {
        Self {
            attack,
            decay,
            sustain,
            sustain_level,
            release,
        }
    }

    pub fn total_duration(&self) -> f32 {
        self.attack + self.decay + self.sustain + self.release
    }
}

impl Default for AdsrGenerator {
    fn default() -> Self {
        Self::new(0.0, 0.0, 1.0, 0.0, 0.0)
    }
}

impl Generator for AdsrGenerator {
    fn generate(&mut self, sample_timing: &SampleTiming) -> PolySample {
        let mut sample_clock = sample_timing.sample_clock();

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
        poly_sample!([value])
    }
}
