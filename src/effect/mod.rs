mod delay;
mod oscilloscope;
use crate::SampleTiming;
pub use delay::Delay;
pub use oscilloscope::Oscilloscope;

pub trait Effect {
    fn process(&mut self, sample_timing: &SampleTiming, sample: Vec<f32>) -> Vec<f32>;
}
