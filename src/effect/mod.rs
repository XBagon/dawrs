mod delay;
mod oscilloscope;
use crate::{SampleTiming, PolySample};
pub use delay::Delay;
pub use oscilloscope::Oscilloscope;

pub trait Effect {
    fn process(&mut self, sample_timing: &SampleTiming, poly_sample: PolySample) -> PolySample;
}
