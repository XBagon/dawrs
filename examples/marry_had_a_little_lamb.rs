use dawrs::{
    effect::{Delay, Oscilloscope},
    generator::{AdsrGenerator, TriangleGenerator},
    prelude::*,
    synthesizer::BasicSynthesizer,
};

#[derive(Default)]
struct MelodyPatch {
    synth: BasicSynthesizer<TriangleGenerator>,
    delay: Delay,
    melody: Vec<u8>,                //list of notes
    note_lengths: Vec<u8>,          //list of note lengths
    melody_index: usize,            //tracks which note is playing
    current_note_quarter_count: u8, //duration of current note
    oscilloscope: Oscilloscope,
}

impl Patch for MelodyPatch {
    fn next_sample(&mut self, sample_timing: &SampleTiming) -> PolySample {
        let quarter_duration = 0.4; //quarter notes of 0.4 seconds

        if sample_timing.every_time_interval(quarter_duration) {
            if self.melody_index < self.melody.len() {
                //every quarter note
                //let quarter_count = (clock % (quarter_length * self.melody.len())) / quarter_length;
                let note = self.melody[self.melody_index];
                let note_length = self.note_lengths[self.melody_index];
                if self.current_note_quarter_count == 0 {
                    //should play new tone
                    self.synth.base_generator.frequency = midi_id_to_frequency(note); //set frequency of synth to right note
                    self.synth.play(quarter_duration * note_length as f32 - 0.5 * quarter_duration);
                    //play note for duration
                }
                self.current_note_quarter_count += 1; //increase amount of quarter notes current note is playing
                if note_length == self.current_note_quarter_count {
                    //end of current note reached
                    self.current_note_quarter_count = 0;
                    self.melody_index += 1;
                }
            } else {
                //after end of last note
                self.current_note_quarter_count += 1;
                //wait time for last note die out
                if self.current_note_quarter_count == 6 {
                    //self.oscilloscope.plot("oscilloscope_output/marry_had_a_little_lamb.png").unwrap(); //uncomment to plot song
                    return poly_sample!(); //return empty sample to stop program
                }
            }
        }

        let mut poly_sample = self.synth.next_sample(&sample_timing);
        poly_sample = self.delay.process(&sample_timing, poly_sample);
        poly_sample = self.oscilloscope.process(&sample_timing, poly_sample);
        poly_sample.polify(2); //make stereo

        poly_sample
    }
}

fn midi_id_to_frequency(midi_id: u8) -> f32 {
    (2.0f32).powf((midi_id - 69) as f32 / 12.0) * 440.0
}

fn main() {
    let mut cpal = Cpal::new().unwrap(); //manages playback

    let mut master_patch = MasterPatch::default(); //patch that easily combines multiple patches and can be "played"

    let patch = MelodyPatch {
        synth: BasicSynthesizer::new(
            Default::default(),
            AdsrGenerator::new(0.05, 0.05, 0.7, 0.2, 0.1),
            0.1,
        ),
        delay: Delay::new(0.3, 0.5),
        melody: vec![
            76, 74, 72, 74, 76, 76, 76, 74, 74, 74, 76, 79, 79, 76, 74, 72, 74, 76, 76, 76, 76, 74,
            74, 76, 74, 72,
        ],
        note_lengths: vec![
            1, 1, 1, 1, 1, 1, 2, 1, 1, 2, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2,
        ],
        oscilloscope: Oscilloscope::new(14.0, 0.001, 0, 512, 1.0),
        ..MelodyPatch::default()
    };

    master_patch.add_patch(patch);

    cpal.play_patch(&mut master_patch);
}
