mod sine;
mod triangle;
pub use sine::SineGenerator;
pub use triangle::TriangleGenerator;

use crate::patch::SampleTiming;

pub trait Generator {
    fn generate(&mut self, sample_timing: &SampleTiming) -> Vec<f32>;
}
