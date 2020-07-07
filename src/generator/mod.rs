mod sine;
mod triangle;
pub use sine::SineGenerator;
pub use triangle::TriangleGenerator;

use crate::patch::SampleTiming;

pub trait Generator {
    fn generate(&mut self, sample_timing: &SampleTiming) -> Vec<f32>;
    fn sample_clock_from_frequency(&mut self, sample_timing: &SampleTiming, frequency: f32) -> f32 {
        let sample_clock_length =
            (1.0 / frequency * sample_timing.sample_rate * frequency.ceil()) as usize; //TODO: Maybe cache?
        (sample_timing.clock % sample_clock_length) as f32
    }
    fn sample_clock_from_length(&mut self, sample_timing: &SampleTiming, length: f32) -> f32 {
        let sample_clock_length =
            (length * sample_timing.sample_rate * (1.0 / length).ceil()) as usize; //TODO: Maybe cache?
        (sample_timing.clock % sample_clock_length) as f32
    }
}
