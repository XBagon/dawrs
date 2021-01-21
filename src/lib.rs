//! DAW-like music/sound production library.
//!
//! ## Usecases
//! * Sound Design with complete freedom
//! * Procedural music
//! * Integrating into games, or other interactive applications
//! * Just "normal" music production
//! * **other weird experiments**
//!
//! ## Features
//!
//! ### Current Features
//! * Generators (Generating waveforms)
//!     * Sine
//!     * Triangle
//!     * ADSR (Attack-Delay-Sustain-Release)
//!     * **more to come**
//!     * **easily expandable**
//! * Audio Effects (Transforming audio)
//!     * Delay
//!     * Lag
//!     * Oscilloscope (Allows easy plotting of the waveform)
//!     * **more to come**
//!     * **easily expandable**
//! * Synthesizer
//!     * BasicSynthesizer (Simplified API of playing sounds, **will be expanded**)
//!     * **more to come**
//!     * **expandable**
//! * Patches for combining and connecting components
//!
//! ### Planned Features
//! * Audio File Support
//!     * Sample "Generator" (which plays samples from file)
//!     * Exporting audio
//! * Expanded I/O support
//!     * Improve Interface, expose more functionality of [`cpal`](https://crates.io/crates/cpal)
//!     * VST support (to easily create VST plugins)
//!     * Allow audio input
//!     * MIDI Controller Input
//!     * Keyboard Input
//! * Song Notation Format
//!     * Should support most features of this library
//!     * Easily readable/writable
//!     * MIDI conversion/integration?
//! * Music Theory Helpers (probably gonna use [`rust-music-theory`](https://crates.io/crates/rust-music-theory) and expand on it)
//!     * Scales
//!     * Chords
//!     * Note Lengths
//!     * Intervals
//!     * Tunings
//!
//! ### Dream Features (time intensive and low priority ideas)
//! * Scripting (Custom language or existing one with simple interface to this library)
//! * UI
//!     * Node-Editor UI to connect components and their parameters visually
//!     * Sequencer
//!     * ..everything else DAW
//! * Hardware Synthesizer running `DAWrs`
//! * Release an executable procedural album (Not really a feature, but my dream and big inspiration behind this)
//!
//! ## Get Started
//!
//! ```
//! use dawrs::{
//!     generator::{SineGenerator, AdsrGenerator},
//!     prelude::*,
//!     synthesizer::BasicSynthesizer,
//! };
//!
//! #[derive(Default)]
//! struct IntroPatch {
//!     sine_synth: BasicSynthesizer<SineGenerator>
//! }
//!
//! impl Patch for IntroPatch {
//!     fn next_sample(&mut self, sample_timing: &SampleTiming) -> PolySample {
//!         if sample_timing.is_time(0.0) { //initially
//!             self.sine_synth.play(1.0); //play for 1 second
//!         } else if sample_timing.is_time(1.05) { //finished note plus its release
//!             return poly_sample![] //returning an empty PolySample stops the patch
//!         }
//!         let mut poly_sample = self.sine_synth.next_sample(sample_timing);
//!         poly_sample.polify(2); //make stereo
//!         poly_sample
//!     }
//! }
//!
//! fn main() {
//! let mut cpal = Cpal::new().unwrap(); //manages playback, uses default playback device. If you need more options, you have to construct it yourself at the moment.
//!
//!     let mut master_patch = MasterPatch::default(); //patch that easily combines multiple patches and can be "played"
//!     let patch = IntroPatch {
//!         sine_synth: BasicSynthesizer::new(
//!             SineGenerator::new(261.626), //set frequency to Middle C
//!             AdsrGenerator::new(0.2, 0.0, 1.0, 0.1, 0.05), 0.1) //configure ADSR so there's no clicking sound
//!     };
//!     master_patch.add_patch(patch);
//!     cpal.play_patch(&mut master_patch);
//! }
//! ```
//!
//! **Look at further [examples](https://github.com/XBagon/dawrs/tree/master/examples)!**

mod cpal;
pub mod effect;
pub mod generator;
pub mod patch;
mod poly_sample;
mod sample_timing;
pub mod synthesizer;

pub use crate::cpal::Cpal;
pub use poly_sample::PolySample;
pub use sample_timing::SampleTiming;

pub mod prelude {
    pub use crate::{
        effect::Effect, generator::Generator, patch::*, poly_sample, Cpal, PolySample, SampleTiming,
    };
}

#[cfg(test)]
mod tests {
    use crate::{
        effect::{Delay, Effect, Oscilloscope},
        generator::{AdsrGenerator, Generator, SineGenerator, TriangleGenerator},
        prelude::*,
        synthesizer::BasicSynthesizer,
    };
    use rand::random;

    fn midi_id_to_frequency(midi_id: u8) -> f32 {
        (2.0f32).powf((midi_id - 69) as f32 / 12.0) * 440.0
    }

    #[test]
    fn glide() {
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
            fn next_sample(&mut self, sample_timing: &SampleTiming) -> PolySample {
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

                let mut cv = self.sine_cv.generate(&sample_timing);
                //cv map from [-1,1] to [0,1]
                cv.linear_map(-1.0..1.0, 0.0..1.0);
                let second_channel = 1.0 - cv[0];
                cv.push(second_channel);

                //cv pans lead
                lead * cv
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

        cpal.play_patch(&mut master_patch);
    }

    #[test]
    fn mary_had_a_little_lamb_chordified() {
        #[derive(Default, Clone)]
        struct MyPatch {
            synth: BasicSynthesizer<TriangleGenerator>,
            delay: Delay,
            melody: Vec<u8>,
            note_lengths: Vec<u8>,
            melody_index: usize,
            current_note_quarter_count: u8,
        }

        impl Patch for MyPatch {
            fn next_sample(&mut self, sample_timing: &SampleTiming) -> PolySample {
                //quarter notes of 0.4 seconds
                let quarter_duration = 0.4;
                let quarter_sample_count = (sample_timing.sample_rate * quarter_duration) as usize;

                let clock = sample_timing.clock;

                if clock % quarter_sample_count == 0 {
                    //let quarter_count = (clock % (quarter_length * self.melody.len())) / quarter_length;
                    let note_length = self.note_lengths[self.melody_index];
                    if note_length == self.current_note_quarter_count {
                        self.current_note_quarter_count = 0;
                        self.melody_index += 1;
                        if self.melody_index == self.melody.len() {
                            self.melody_index = 0;
                        }
                    }
                    if self.current_note_quarter_count == 0 {
                        let note_length = self.note_lengths[self.melody_index];
                        let note_length = note_length as f32;
                        self.synth.play(quarter_duration * note_length as f32 - 0.2);
                    }
                    self.current_note_quarter_count += 1;
                }

                let note = self.melody[self.melody_index];

                self.synth.base_generator.frequency = midi_id_to_frequency(note);
                let mut poly_sample = self.synth.next_sample(&sample_timing) * (1.0 / 3.0);

                self.synth.base_generator.frequency = midi_id_to_frequency(note + 4);
                poly_sample += self.synth.next_sample(&sample_timing) * (1.0 / 3.0);

                self.synth.base_generator.frequency = midi_id_to_frequency(note + 7);
                poly_sample += self.synth.next_sample(&sample_timing) * (1.0 / 3.0);

                //make stereo
                poly_sample.polify(2);

                poly_sample
            }
        }

        let mut cpal = Cpal::new().unwrap();

        let mut master_patch = MasterPatch::default();

        let patch = MyPatch {
            synth: BasicSynthesizer::new(
                TriangleGenerator::default(),
                AdsrGenerator::new(0.05, 0.05, 0.7, 0.2, 0.1),
                0.1,
            ),
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

        cpal.play_patch(&mut master_patch);
    }

    #[test]
    fn plot() {
        struct MyPatch {
            sine_gen: SineGenerator,
            sine_oscope: Oscilloscope,
            adsr_gen: AdsrGenerator,
            adsr_oscope: Oscilloscope,
            triangle_gen: TriangleGenerator,
            triangle_oscope: Oscilloscope,
        }

        const FREQ: f32 = 440.0;
        const DURATION: f32 = 1.0 / FREQ;

        impl Patch for MyPatch {
            fn next_sample(&mut self, sample_timing: &SampleTiming) -> PolySample {
                let mut sine = self.sine_gen.generate(&sample_timing);
                sine = self.sine_oscope.process(&sample_timing, sine);

                let mut triangle = self.triangle_gen.generate(&sample_timing);
                triangle = self.triangle_oscope.process(&sample_timing, triangle);

                let sample_count = sample_timing.duration_to_sample_count(DURATION);

                if sample_timing.clock == sample_count - 1 {
                    self.sine_oscope.plot("oscilloscope_output/sine.png").unwrap();
                    self.triangle_oscope.plot("oscilloscope_output/triangle.png").unwrap();
                }

                let mut adsr = self.adsr_gen.generate(&sample_timing);
                adsr = self.adsr_oscope.process(&sample_timing, adsr);

                let sample_count = sample_timing.duration_to_sample_count(0.05 + 0.05 + 0.2 + 0.1);

                if sample_timing.clock == sample_count - 1 {
                    self.adsr_oscope.plot("oscilloscope_output/adsr.png").unwrap();
                    std::process::exit(0);
                }

                let sample_value = sine[0] * triangle[0] * adsr[0] * 0.1;
                poly_sample!([sample_value; 2])
            }
        }

        let mut cpal = Cpal::new().unwrap();

        let mut master_patch = MasterPatch::default();

        let patch = MyPatch {
            sine_gen: SineGenerator::new(FREQ),
            sine_oscope: Oscilloscope::new(DURATION, DURATION / 1000.0, 0, 512, 1000.0),
            triangle_gen: TriangleGenerator::new(FREQ),
            triangle_oscope: Oscilloscope::new(DURATION, DURATION / 1000.0, 0, 512, 1000.0),
            adsr_gen: AdsrGenerator::new(0.05, 0.05, 0.7, 0.2, 0.1),
            adsr_oscope: Oscilloscope::new(0.05 + 0.05 + 0.2 + 0.1, 0.01, 0, 512, 1.0),
        };

        master_patch.add_patch(patch);

        cpal.play_patch(&mut master_patch);
    }
}
