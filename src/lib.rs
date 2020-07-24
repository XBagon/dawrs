pub mod cpal;
pub mod effect;
pub mod generator;
pub mod patch;
pub mod sample_timing;

pub use crate::cpal::Cpal;
pub use sample_timing::SampleTiming;

pub mod prelude {
    pub use crate::{patch::*, Cpal, SampleTiming};
}

#[cfg(test)]
mod tests {
    use crate::{
        effect::{Delay, Effect},
        generator::{AdsrGenerator, Generator, SineGenerator, TriangleGenerator},
        prelude::*,
    };

    fn midi_id_to_frequency(midi_id: u8) -> f32 {
        (2 as f32).powf((midi_id - 69) as f32 / 12.0) * 440.0
    }

    #[test]
    fn glide_test() {
        #[derive(Default, Clone)]
        struct MyPatch {
            time_offset: f32,
            original_frequency: f32,
            target_frequency: f32,
            glide_length: usize,
            triangle_synth: TriangleGenerator,
            sine_cv: SineGenerator,
        }

        impl Patch for MyPatch {
            fn next_value(&mut self, sample_timing: &SampleTiming) -> Vec<f32> {
                //bar of 1.5 seconds
                let bar = (sample_timing.sample_rate * 1.5) as usize;
                let offset = (sample_timing.sample_rate * self.time_offset) as usize;

                let clock = sample_timing.clock + offset;

                if clock % bar == 0 {
                    if clock != 0 {
                        if clock % (bar * 6) == 0 {
                            self.original_frequency = self.triangle_synth.frequency;
                            self.target_frequency = self.original_frequency / 3.0;
                            self.glide_length = 24000;
                        } else {
                            self.original_frequency = self.triangle_synth.frequency;
                            self.target_frequency = self.original_frequency * 1.2;
                            self.glide_length = 24000;
                        }
                    }
                }

                if self.glide_length > 0 {
                    self.triangle_synth.frequency = self.original_frequency
                        + (1.0 - (self.glide_length as f32 / 24000.0))
                            * (self.target_frequency - self.original_frequency);
                    self.glide_length -= 1;
                }

                let mut lead = self.triangle_synth.generate(&sample_timing)[0];
                //turn volume down
                lead *= 0.1;

                //cv map from [-1,1] to [0,1]
                let cv = (self.sine_cv.generate(&sample_timing)[0] + 1.0) / 2.0;

                //cv pans lead
                vec![lead * cv, lead * (1.0 - cv)]
            }
        }

        let mut cpal = Cpal::new().unwrap();

        let mut master_patch = MasterPatch::default();

        let patch = MyPatch {
            sine_cv: SineGenerator::new(0.3),
            ..MyPatch::default()
        };

        let mut patch2 = patch.clone();
        //second synth starts on B instead of A
        patch2.triangle_synth.frequency = 493.88;
        //offset of 1 second
        patch2.time_offset = 1.0;

        master_patch.add_patch(patch);
        //master_patch.add_patch(patch2);

        master_patch.play(&mut cpal, SampleTiming::default());
    }

    #[test]
    fn marry_had_a_little_lamb() {
        #[derive(Default, Clone)]
        struct MyPatch {
            triangle_synth: TriangleGenerator,
            adsr: AdsrGenerator,
            delay: Delay,
            melody: Vec<u8>,
            note_lengths: Vec<u8>,
            melody_index: usize,
            current_note_quarter_count: u8,
        }

        impl Patch for MyPatch {
            fn next_value(&mut self, sample_timing: &SampleTiming) -> Vec<f32> {
                //quarter notes of 0.4 seconds
                let quarter_duration = 0.4;
                let quarter_sample_count = (sample_timing.sample_rate * quarter_duration) as usize;

                let clock = sample_timing.clock;

                if clock % quarter_sample_count == 0 {
                    //let quarter_count = (clock % (quarter_length * self.melody.len())) / quarter_length;
                    let note = self.melody[self.melody_index];
                    let note_length = self.note_lengths[self.melody_index];
                    if self.current_note_quarter_count == 0 {
                        self.triangle_synth.frequency = midi_id_to_frequency(note);
                        self.triangle_synth.start_tick = clock;
                        let note_length = note_length as f32;
                        self.adsr.sustain = quarter_duration * note_length - 0.2; //0.2s are a+d+r
                        self.adsr.start_tick = clock;
                    }
                    self.current_note_quarter_count += 1;
                    if note_length == self.current_note_quarter_count {
                        self.current_note_quarter_count = 0;
                        self.melody_index += 1;
                        if self.melody_index == self.melody.len() {
                            self.melody_index = 0;
                        }
                    }
                }

                let mut lead = self.triangle_synth.generate(&sample_timing)[0];
                //turn volume down
                lead *= 0.1;

                let adsr_value = self.adsr.generate(&sample_timing)[0];

                //ADSR controls volume
                let sample = vec![lead * adsr_value, lead * adsr_value];

                self.delay.process(&sample_timing, sample)
            }
        }

        let mut cpal = Cpal::new().unwrap();

        let mut master_patch = MasterPatch::default();

        let patch = MyPatch {
            adsr: AdsrGenerator::new(0.05, 0.05, 0.7, 0.2, 0.1),
            delay: Delay::new(0.3, 0.5),
            melody: vec![
                76, 74, 72, 74, 76, 76, 76, 74, 74, 74, 76, 79, 79, 76, 74, 72, 74, 76, 76, 76, 76,
                74, 74, 76, 74, 72,
            ],
            note_lengths: vec![
                1, 1, 1, 1, 1, 1, 2, 1, 1, 2, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 4,
            ],
            ..MyPatch::default()
        };

        master_patch.add_patch(patch);

        master_patch.play(&mut cpal, SampleTiming::default());
    }
}
