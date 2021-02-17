use super::Effect;
use crate::prelude::*;
use std::collections::VecDeque;
use rustfft::{FftPlanner, Fft};
use rustfft::num_complex::Complex;
use std::sync::Arc;


#[derive(Clone)]
pub struct HighpassFilter {
    pub cutoff: f32,
    pub length: usize,
    fft_forward: Arc<dyn Fft<f32>>,
    fft_inverse: Arc<dyn Fft<f32>>,
    buffer: VecDeque<Complex<f32>>,
    output_buffer: Vec<Complex<f32>>,
}

impl HighpassFilter {
    pub fn new(cutoff: f32, length: usize) -> Self {
        Self {
            cutoff,
            length,
            fft_forward: FftPlanner::new().plan_fft_forward(length),
            fft_inverse: FftPlanner::new().plan_fft_inverse(length),
            buffer: VecDeque::with_capacity(length),
            output_buffer: vec![Complex{ re: 0.0, im: 0.0 }; length],
        }
    }
}

impl Effect for HighpassFilter {
    fn process(&mut self, _: &SampleTiming, poly_sample: PolySample) -> PolySample {
        let mut output = poly_sample!([0.0]);
        if self.buffer.len() == self.length {
            self.output_buffer.copy_from_slice(self.buffer.make_contiguous());
            self.fft_forward.process(&mut self.output_buffer);
            //TODO: FILTER
            //self.fft_inverse.process(&mut self.output_buffer);
            output = poly_sample!([self.output_buffer[0].re/(self.length as f32)]);
            self.buffer.pop_front().unwrap();
        }
        self.buffer.push_back(Complex { re: poly_sample[0], im: 0.0 });
        poly_sample
    }
}

#[cfg(test)]
mod tests {
    use crate::generator::SineGenerator;
    use crate::prelude::*;
    use crate::effect::Oscilloscope;
    use crate::effect::filter::HighpassFilter;
    use plotters::prelude::*;

    const SAMPLE_RATE: f32 = 48_000.0;
    const LENGTH: usize = 1024;

    struct MyPatch {
        pub generator: SineGenerator,
        pub filter: HighpassFilter,
    }

    impl MyPatch {
        pub fn new() -> Self {
            Self {
                generator: SineGenerator::new(440.0),
                filter: HighpassFilter::new(100.0, LENGTH),
            }
        }
    }

    impl Patch for MyPatch {
        fn next_sample(&mut self, sample_timing: &SampleTiming) -> PolySample {
            let poly_sample = self.generator.generate(&sample_timing);
            self.filter.process(&sample_timing, poly_sample)
        }
    }

    #[test]
    fn test() {
        let mut sample_timing = SampleTiming::new(48_000.0);
        let mut patch = MyPatch::new();
        let mut oscilloscope = Oscilloscope::new((LENGTH as f32)/SAMPLE_RATE, 1.0/SAMPLE_RATE, 0, 512, 50.0);

        for _ in 0..LENGTH*2 {
            let poly_sample = patch.next_sample(&sample_timing);
            oscilloscope.process(&sample_timing, poly_sample);
            sample_timing.tick();
        }
        oscilloscope.plot("oscilloscope_output/filter_test_wave.png").unwrap();

        patch.filter.process(&sample_timing, poly_sample!([0.0]));

        let root = BitMapBackend::new("oscilloscope_output/filter_test_fft.png", (640, 480)).into_drawing_area();
        root.fill(&WHITE).unwrap();
        let mut chart = ChartBuilder::on(&root)
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(
                0.0f32..(LENGTH as f32/2.0),
                -10.0f32..10.0f32,
            ).unwrap();
        chart.configure_mesh().draw().unwrap();
        chart
            .draw_series(LineSeries::new(
                patch.filter.output_buffer.iter().take(LENGTH/2).enumerate().map(|(i, z)| (i as f32 * (48_000.0/LENGTH as f32), z.re/(LENGTH as f32).sqrt())),
                &RED,
            )).unwrap();
    }
}