mod adsr;
mod sine;
mod triangle;
pub use adsr::AdsrGenerator;
pub use sine::SineGenerator;
pub use triangle::TriangleGenerator;

use crate::{SampleTiming, PolySample};

pub trait Generator {
    fn generate(&mut self, sample_timing: &SampleTiming) -> PolySample;
}
