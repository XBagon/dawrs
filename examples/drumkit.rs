use dawrs::{
    effect::Delay,
    generator::{AdsrGenerator, TriangleGenerator},
    prelude::*,
    synthesizer::BasicSynthesizer,
};
use rand::random;

fn main() {
    #[derive(Default, Clone)]
    struct MyPatch {
        kick: BasicSynthesizer<TriangleGenerator>,
        clave: BasicSynthesizer<TriangleGenerator>,
        delay: Delay,
    }

    impl Patch for MyPatch {
        fn next_sample(&mut self, sample_timing: &SampleTiming) -> PolySample {
            let beats_per_minute = 140;
            let beats_per_bar = 4;
            let bar_duration = (60.0 * beats_per_bar as f32) / beats_per_minute as f32;

            let bar_samples =  (sample_timing.sample_rate * bar_duration) as usize; //count of samples in bar
            let quarter_samples = bar_samples / beats_per_bar;
            let eighth_samples = quarter_samples / 2;
            let sixteenth_samples = eighth_samples / 2;
            let quarter_triplet_samples = quarter_samples / 3;

            let clock = sample_timing.clock;

            if clock % sixteenth_samples == 0 { //every sixteenth note
                if clock % quarter_samples == 0 { //every fourth sixteenth note -> every quarter note
                    if clock % bar_samples == quarter_samples * 0 { //first note every bar
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
                    == eighth_samples * 3 //every 3rd eigth note
                {
                    self.kick.base_generator.frequency = 60.0;
                    self.kick.play(0.01);

                }
            }

            if clock % quarter_triplet_samples == 0 { //every quarter triplet note
                if clock > bar_samples * 4 { //starts after 4 bars
                    self.clave.play(0.004);
                }
            }

            let mut poly_sample = self.kick.next_sample(&sample_timing);
            poly_sample = self.delay.process(&sample_timing, poly_sample);
            poly_sample += self.clave.next_sample(&sample_timing) * random(); //simple distortion

            poly_sample.polify(2);

            poly_sample
        }
    }

    let mut cpal = Cpal::new().unwrap();

    let mut master_patch = MasterPatch::default();

    let patch = MyPatch {
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
        ..MyPatch::default()
    };

    master_patch.add_patch(patch);

    cpal.play_patch(&mut master_patch);
}
