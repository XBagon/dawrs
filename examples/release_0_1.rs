use dawrs::prelude::*;
use dawrs::synthesizer::BasicSynthesizer;
use dawrs::generator::AdsrGenerator;

struct ReleasePatch {
    synth: BasicSynthesizer<fn(&SampleTiming) -> PolySample>,
}

impl Patch for ReleasePatch {
    fn next_sample(&mut self, sample_timing: &SampleTiming) -> PolySample {
        if sample_timing.is_time(0.0) {self.synth.play(4.0);}
        let mut poly_sample = self.synth.next_sample(sample_timing);
        poly_sample
    }
}

fn main() {
    let mut cpal = Cpal::new().unwrap(); //manages playback

    let mut master_patch = MasterPatch::default(); //patch that easily combines multiple patches and can be "played"

    let patch = ReleasePatch {
        synth: BasicSynthesizer::new(
            |sample_timing| {
                let sample_clock = sample_timing.sample_clock();
                let frequency = 100.0 + (sample_clock * 10.0).sin() * 20.0;
                let value = (frequency * 2.0 * std::f32::consts::PI).sin().max(-0.7)-0.1;
                poly_sample!([value*value, (1.0-value.abs())*value])
            },
            AdsrGenerator::new(0.2, 0.1, 0.9, 0.5, 0.3),
            0.1,
        ),
    };

    master_patch.add_patch(patch);

    cpal.play_patch(&mut master_patch);
}