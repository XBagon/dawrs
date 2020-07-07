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
            sine_synth: TriangleGenerator,
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
                            self.sine_synth.frequency *= 0.3;
                        } else {
                            self.sine_synth.frequency *= 1.2;
                        }
                        dbg!(self.sine_synth.frequency);
                    }
                }

                let lead = self.sine_synth.generate(&sample_timing);
                let cv = self.sine_cv.generate(&sample_timing);
                //cv controls volume of the lead
                lead.into_iter().zip(cv).map(|(lead, cv)| lead * cv * 0.1).collect()
            }
        }

        let mut cpal = Cpal::new().unwrap();

        let mut master_patch = MasterPatch::default();

        let patch = MyPatch {
            sine_cv: SineGenerator::new(1.0),
            ..MyPatch::default()
        };

        let mut patch2 = patch.clone();
        //second synth starts on B instead of A
        patch2.sine_synth.frequency = 493.88;
        //offset of 1 second
        patch2.time_offset = 1.0;

        master_patch.add_patch(patch);
        master_patch.add_patch(patch2);

        master_patch.play(&mut cpal, SampleTiming::default());
    }
}
