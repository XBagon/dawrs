use super::Effect;
use crate::prelude::*;
use std::collections::VecDeque;
use rustfft::{FftPlanner, Fft};
use rustfft::num_complex::Complex;
use std::sync::Arc;
use rustfft::num_traits::Zero;


#[derive(Clone)]
pub struct LowPassFilter {
    pub cutoff: f32,
    pub length: usize,
    fft_forward: Arc<dyn Fft<f32>>,
    fft_inverse: Arc<dyn Fft<f32>>,
    buffer: VecDeque<Complex<f32>>,
    output_buffer: Vec<Complex<f32>>,
}

impl LowPassFilter {
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

impl Effect for LowPassFilter {
    fn process(&mut self, sample_timing: &SampleTiming, poly_sample: PolySample) -> PolySample {
        let mut output = poly_sample!([0.0]);
        if self.buffer.len() == self.length {
            self.output_buffer.copy_from_slice(self.buffer.make_contiguous());
            self.fft_forward.process(&mut self.output_buffer);
            for (i, mut z) in self.output_buffer.iter_mut().enumerate() {
                let x = i as f32 * (sample_timing.sample_rate/self.length as f32);
                //let y = z.re/(LENGTH as f32).sqrt();
                //if x > self.cutoff {
                    *z *= 1.0 - (x / self.cutoff).min(1.0);
                //}
            }
            self.fft_inverse.process(&mut self.output_buffer);
            output = poly_sample!([self.output_buffer[0].re/(self.length as f32)]);
            self.buffer.pop_front().unwrap();
        }
        self.buffer.push_back(Complex { re: poly_sample[0], im: 0.0 });
        output
    }
}

#[cfg(test)]
mod tests {
    use crate::generator::SineGenerator;
    use crate::prelude::*;
    use crate::effect::Oscilloscope;
    use crate::effect::filter::LowPassFilter;
    use plotters::prelude::*;

    const SAMPLE_RATE: f32 = 48_000.0;
    const LENGTH: usize = 1024;

    struct MyPatch {
        pub generator: fn(&SampleTiming) -> PolySample,
        pub filter: LowPassFilter,
    }

    impl MyPatch {
        pub fn new() -> Self {
            Self {
                generator: |sample_timing: &SampleTiming| {
                    let clock = sample_timing.sample_clock();
                    let y = (clock * 440.0 * 2.0 * std::f32::consts::PI).sin() + (clock * 247.5 * 2.0 * std::f32::consts::PI).sin() + (clock * 874.5 * 2.0 * std::f32::consts::PI).sin();
                    poly_sample!([y * 0.1])
                },
                filter: LowPassFilter::new(900.0, LENGTH),
            }
        }
    }

    impl Patch for MyPatch {
        fn next_sample(&mut self, sample_timing: &SampleTiming) -> PolySample {
            self.filter.cutoff = 500.0 + (sample_timing.sample_clock() * 1.0 * 4.0 * std::f32::consts::PI).sin() * 400.0;
            let mut poly_sample = self.generator.generate(&sample_timing);
            poly_sample = self.filter.process(&sample_timing, poly_sample);
            poly_sample.polify(2);
            poly_sample
        }
    }

    #[test]
    fn test() {
        let mut sample_timing = SampleTiming::new(48_000.0);
        let mut patch = MyPatch::new();
        let mut oscilloscope = Oscilloscope::new(0.1, 1.0/SAMPLE_RATE, 0, 512, 50.0);

        for _ in 0..LENGTH*8 {
            let poly_sample = patch.next_sample(&sample_timing);
            oscilloscope.process(&sample_timing, poly_sample);
            sample_timing.tick();
        }
        oscilloscope.plot("oscilloscope_output/filter_test_wave.png").unwrap();
    }

    #[test]
    fn test_sound() {
        let mut cpal = Cpal::new().unwrap();
        let mut master_patch = MasterPatch::default();
        master_patch.add_patch(MyPatch::new());
        cpal.play_patch(&mut master_patch);
    }
}