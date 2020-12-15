mod delay;
mod lag;
mod oscilloscope;

use crate::{PolySample, SampleTiming};
pub use delay::Delay;
pub use lag::Lag;
pub use oscilloscope::Oscilloscope;

pub trait Effect: Send {
    fn process(&mut self, sample_timing: &SampleTiming, poly_sample: PolySample) -> PolySample;
}
