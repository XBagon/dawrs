mod adsr;
mod sine;
mod triangle;
pub use adsr::AdsrGenerator;
pub use sine::SineGenerator;
pub use triangle::TriangleGenerator;

use crate::{PolySample, SampleTiming};

pub trait Generator: Send {
    fn generate(&mut self, sample_timing: &SampleTiming) -> PolySample;
}

impl<T: FnMut(&SampleTiming) -> PolySample + Send> Generator for T {
    fn generate(&mut self, sample_timing: &SampleTiming) -> PolySample {
        self(sample_timing)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn generator_from_closure() {
        let mut x = 1.0;
        let mut generator = move |_: &SampleTiming| {
            x *= 0.5;
            poly_sample!([x])
        };
        let sample_timing = &SampleTiming {
            sample_rate: 0.0,
            clock: 0,
        };
        assert_abs_diff_eq!(generator.generate(sample_timing).0[0], 0.5);
        assert_abs_diff_eq!(generator.generate(sample_timing).0[0], 0.5 * 0.5);
    }
}
