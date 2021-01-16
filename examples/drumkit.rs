use dawrs::{
    effect::Delay,
    generator::{AdsrGenerator, TriangleGenerator},
    prelude::*,
    synthesizer::BasicSynthesizer,
};
use rand::random;

#[derive(Default, Clone)]
struct DrumKit {
    kick: BasicSynthesizer<TriangleGenerator>,
    clave: BasicSynthesizer<TriangleGenerator>,
    delay: Delay,
}

impl Patch for DrumKit {
    fn next_sample(&mut self, sample_timing: &SampleTiming) -> PolySample {
        let sixteenth_duration = 0.1; //start with smallest note length, so multiples of the duration are also multiples of the sample count
        let sixteenth_samples = sample_timing.duration_to_sample_count(sixteenth_duration); //count of samples in one sixteenth note

        let eighth_samples = sixteenth_samples * 2;
        let quarter_samples = eighth_samples * 2;
        let bar_samples = quarter_samples * 4; //count of samples in bar
        let quarter_triplet_samples = quarter_samples / 3;

        let clock = sample_timing.clock;

        if clock % sixteenth_samples == 0 {
            //every sixteenth note
            if clock % quarter_samples == 0 {
                //every fourth sixteenth note -> every quarter note
                if clock % bar_samples == quarter_samples * 0 {
                    //first note every bar
                    self.kick.volume = 0.2; //emphasize first note
                } else {
                    self.kick.volume = 0.1; //don't emphasize others
                }
                self.kick.base_generator.frequency = 40.0;
                self.kick.play(0.01);
            }
            //`else` possible here, but easy to break when changing timing of notes
            if clock
                % bar_samples //every bar
                == eighth_samples * 3
            //every 3rd eigth note
            {
                self.kick.base_generator.frequency = 60.0;
                self.kick.play(0.01);
            }
        }

        if clock % quarter_triplet_samples == 0 {
            //every quarter triplet note
            if clock > bar_samples * 4 {
                //starts after 4 bars
                self.clave.play(0.004);
            }
        }

        let mut poly_sample = self.kick.next_sample(&sample_timing);
        poly_sample = self.delay.process(&sample_timing, poly_sample);
        poly_sample += self.clave.next_sample(&sample_timing) * random(); //simple distortion
        poly_sample.polify(2); //make stereo

        poly_sample
    }
}

fn main() {
    let mut cpal = Cpal::new().unwrap(); //manages playback

    let mut master_patch = MasterPatch::default(); //patch that easily combines multiple patches and can be "played"

    let patch = DrumKit {
        kick: BasicSynthesizer::new(
            TriangleGenerator::new(40.0),
            AdsrGenerator::new(0.001, 0.05, 0.9, 0.1, 0.04),
            0.1,
        ),
        clave: BasicSynthesizer::new(
            TriangleGenerator::new(1200.0),
            AdsrGenerator::new(0.001, 0.001, 0.9, 0.1, 0.03),
            0.1,
        ),
        delay: Delay::new(0.1, 0.3),
        ..DrumKit::default()
    };

    master_patch.add_patch(patch);

    cpal.play_patch(&mut master_patch);
}
