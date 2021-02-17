use dawrs::prelude::*;
use dawrs::synthesizer::BasicSynthesizer;
use dawrs::generator::{AdsrGenerator, SineGenerator};
use dawrs::effect::Delay;
use rand::{thread_rng, Rng};

struct ReleasePatch {
    texture_synth: BasicSynthesizer<TextureGenerator>,
    lead_synth: BasicSynthesizer<SineGenerator>,
    delay: Delay,
    current_note_length: u32,
}

impl Patch for ReleasePatch {
    fn next_sample(&mut self, sample_timing: &SampleTiming) -> PolySample {
        if sample_timing.is_time(1.0) {
            //return poly_sample!();
        } else if sample_timing.is_time(0.0) {
            self.texture_synth.play(60.0);
            self.lead_synth.base_generator.frequency = thread_rng().gen_range(4, 10) as f32 * 330.0;
            self.current_note_length = thread_rng().gen_range(2, 5);
            if thread_rng().gen::<f32>() < 0.1 {
                self.current_note_length = 1;
            }
            self.lead_synth.play(self.current_note_length as f32-0.5);
        }
        if sample_timing.is_after_interval(1.0) {
            self.current_note_length-=1;
            if self.current_note_length == 0 {
                self.lead_synth.base_generator.frequency = thread_rng().gen_range(4, 9) as f32 * 220.0;
                self.current_note_length = thread_rng().gen_range(2, 4);
                self.lead_synth.play(self.current_note_length as f32 - 0.5);
                if thread_rng().gen::<f32>() > 0.1 {
                    self.current_note_length = 1;
                }
            }
        }
        let mut texture_sample = self.texture_synth.next_sample(sample_timing);
        texture_sample = self.delay.process(sample_timing, texture_sample);

        let mut lead_sample= self.texture_synth.next_sample(&(sample_timing+100))*self.lead_synth.next_sample(sample_timing)[0];
        lead_sample = self.delay.process(sample_timing, lead_sample);

        texture_sample + lead_sample
    }
}

struct TextureGenerator {
    state: u64,
    interpolation: f32,
}

//TODO: maybe add sax like thing

impl Generator for TextureGenerator {
    fn generate(&mut self, sample_timing: &SampleTiming) -> PolySample {
        let sample_clock = sample_timing.sample_clock();
        let frequency = 30.0 + (sample_clock * 2.0 + self.interpolation).sin() * 10.0;
        let value = (frequency * 2.0 * std::f32::consts::PI).sin().max(-0.7)-0.1;
        let target = self.state as f32 / u64::MAX as f32;
        if (self.interpolation - target).abs() < 0.00001 {
            self.state = self.state.rotate_right(1) ^ value.to_bits() as u64;
        }
        self.interpolation = self.interpolation + (target-self.interpolation)*(0.004+0.003*(sample_clock*10.0).sin());
        poly_sample!([(1.0-value.abs())*value, (value.abs())*value])
    }
}

fn main() {
    let mut cpal = Cpal::new().unwrap(); //manages playback

    let mut master_patch = MasterPatch::default(); //patch that easily combines multiple patches and can be "played"

    let patch = ReleasePatch {
        texture_synth: BasicSynthesizer::new(TextureGenerator { state: 1, interpolation: 0.0 },
            AdsrGenerator::new(2.0, 0.1, 0.9, 0.5, 0.3),
            0.05,
        ),
        lead_synth: BasicSynthesizer::new(SineGenerator::new(440.0),
                                          AdsrGenerator::new(0.4, 0.0 ,1.0, 0.5, 0.2),
                                          0.4,
        ),
        delay: Delay::new(0.8,0.6),
        current_note_length: 0
    };

    master_patch.add_patch(patch);

    cpal.play_patch(&mut master_patch);
}