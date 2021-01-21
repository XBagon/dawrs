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
* UI
    * Node-Editor UI to connect components and their parameters visually
    * Sequencer
    * ..everything else DAW
* Hardware Synthesizer running `DAWrs`
* Release an executable procedural album (Not really a feature, but my dream and big inspiration behind this)

### Get Started
**Look at the [examples](https://github.com/XBagon/dawrs/tree/master/examples)!**

License: MIT
