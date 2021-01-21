[![Workflow Status](https://github.com/XBagon/dawrs/workflows/main/badge.svg)](https://github.com/XBagon/dawrs/actions?query=workflow%3A%22main%22)

# DAWrs */doors/*

DAW-like music/sound production library.

### Usecases
* Sound Design with complete freedom
* Procedural music
* Integrating into games, or other interactive applications
* Just "normal" music production
* **other weird experiments**

### Features

#### Current Features
* Generators (Generating waveforms)
    * Sine
    * Triangle
    * ADSR (Attack-Delay-Sustain-Release)
    * **more to come**
    * **easily expandable**
* Audio Effects (Transforming audio)
    * Delay
    * Lag
    * Oscilloscope (Allows easy plotting of the waveform)
    * **more to come**
    * **easily expandable**
* Synthesizer
    * BasicSynthesizer (Simplified API of playing sounds, **will be expanded**)
    * **more to come**
    * **expandable**
* Patches for combining and connecting components

#### Planned Features
* Audio File Support
    * Sample "Generator" (which plays samples from file)
    * Exporting audio
* Expanded I/O support
    * Improve Interface, expose more functionality of [`cpal`](https://crates.io/crates/cpal)
    * VST support (to easily create VST plugins)
    * Allow audio input
    * MIDI Controller Input
    * Keyboard Input
* Song Notation Format
    * Should support most features of this library
    * Easily readable/writable
    * MIDI conversion/integration?
* Music Theory Helpers (probably gonna use [`rust-music-theory`](https://crates.io/crates/rust-music-theory) and expand on it)
    * Scales
    * Chords
    * Note Lengths
    * Intervals
    * Tunings

#### Dream Features (time intensive and low priority ideas)
* Scripting (Custom language or existing one with simple interface to this library)
* UI
    * Node-Editor UI to connect components and their parameters visually
    * Sequencer
    * ..everything else DAW
* Hardware Synthesizer running `DAWrs`
* Release an executable procedural album (Not really a feature, but my dream and big inspiration behind this)

### Get Started

```rust
use dawrs::{
    generator::{SineGenerator, AdsrGenerator},
    prelude::*,
    synthesizer::BasicSynthesizer,
};

#[derive(Default)]
struct IntroPatch {
    sine_synth: BasicSynthesizer<SineGenerator>
}

impl Patch for IntroPatch {
    fn next_sample(&mut self, sample_timing: &SampleTiming) -> PolySample {
        if sample_timing.is_time(0.0) { //initially
            self.sine_synth.play(1.0); //play for 1 second
        } else if sample_timing.is_time(1.05) { //finished note plus its release
            return poly_sample!() //returning an empty PolySample stops the patch
        }
        let mut poly_sample = self.sine_synth.next_sample(sample_timing);
        poly_sample.polify(2); //make stereo
        poly_sample
    }
}

fn main() {
let mut cpal = Cpal::new().unwrap(); //manages playback, uses default playback device. If you need more options, you have to construct it yourself at the moment.

    let mut master_patch = MasterPatch::default(); //patch that easily combines multiple patches and can be "played"
    let patch = IntroPatch {
        sine_synth: BasicSynthesizer::new(
            SineGenerator::new(261.626), //set frequency to Middle C
            AdsrGenerator::new(0.2, 0.0, 1.0, 0.1, 0.05), 0.1) //configure ADSR so there's no clicking sound
    };
    master_patch.add_patch(patch);
    cpal.play_patch(&mut master_patch);
}
```

**Look at further [examples](https://github.com/XBagon/dawrs/tree/master/examples)!**

License: MIT
