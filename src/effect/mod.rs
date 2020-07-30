mod delay;
mod oscilloscope;
use crate::{PolySample, SampleTiming};
pub use delay::Delay;
pub use oscilloscope::Oscilloscope;

pub trait Effect: Send {
    fn process(&mut self, sample_timing: &SampleTiming, poly_sample: PolySample) -> PolySample;
}
