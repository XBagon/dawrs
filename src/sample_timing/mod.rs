mod impl_ops;

#[derive(Clone, Copy)]
pub struct SampleTiming {
    pub sample_rate: f32,
    pub clock: usize,
}

impl SampleTiming {
    pub fn new(sample_rate: f32) -> Self {
        SampleTiming {
            sample_rate,
            clock: 0,
        }
    }
}

impl SampleTiming {
    pub fn tick(&mut self) {
        self.clock += 1;
    }

    /// More inaccurate than [`sample_clock_with_frequency`](Self::sample_clock_with_frequency)
    /// and [`sample_clock_with_length`](Self::sample_clock_with_length), but more flexible,
    /// as frequency/length doesn't have to be known beforehand.
    ///
    /// Useful for dynamic length.
    pub fn sample_clock(&self) -> f32 {
        self.clock as f32 / self.sample_rate
    }

    pub fn sample_clock_with_frequency(&self, frequency: f32) -> f32 {
        let sample_clock_length = (1.0 / frequency * self.sample_rate * frequency.ceil()) as usize;
        ((self.clock + sample_clock_length) % sample_clock_length) as f32 / self.sample_rate
    }

    pub fn sample_clock_with_length(&self, length: f32) -> f32 {
        let sample_clock_length = (length * self.sample_rate * (1.0 / length).ceil()) as usize;
        ((self.clock + sample_clock_length) % sample_clock_length) as f32 / self.sample_rate
    }

    pub fn duration_to_sample_count(&self, duration: f32) -> usize {
        (duration * self.sample_rate) as usize
    }
}