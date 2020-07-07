mod cpal;
mod effect;
mod generator;
mod patch;

#[cfg(test)]
mod tests {
    use crate::{
        cpal::Cpal,
        generator::{Generator, SineGenerator, TriangleGenerator},
        patch::{MasterPatch, Patch, SampleTiming},
    };

    #[test]
    fn test() {
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
}
