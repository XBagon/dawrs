#[derive(Clone)]
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

//TODO: remove
impl Default for SampleTiming {
    fn default() -> Self {
        Self {
            sample_rate: 44_000.0,
            clock: 0,
        }
    }
}

impl SampleTiming {
    pub fn tick(&mut self) {
        self.clock += 1;
    }

    /// More inaccurate than [`sample_clock_with_frequency`](#method.sample_clock_with_frequency)
    /// and [`sample_clock_with_length`](#method.sample_clock_with_length), but more flexible,
    /// as frequency/length doesn't have to be known beforehand.
    ///
    /// Useful for dynamic length.
    pub fn sample_clock(&self, start_tick: usize) -> f32 {
        (self.clock - start_tick) as f32 / self.sample_rate
    }

    pub fn sample_clock_with_frequency(&self, frequency: f32, start_tick: usize) -> f32 {
        let sample_clock_length = (1.0 / frequency * self.sample_rate * frequency.ceil()) as usize; //TODO: Maybe cache?
        ((self.clock + sample_clock_length - start_tick % sample_clock_length)
            % sample_clock_length) as f32
            / self.sample_rate
    }

    pub fn sample_clock_with_length(&self, length: f32, start_tick: usize) -> f32 {
        let sample_clock_length = (length * self.sample_rate * (1.0 / length).ceil()) as usize; //TODO: Maybe cache?
        ((self.clock + sample_clock_length - start_tick % sample_clock_length)
            % sample_clock_length) as f32
            / self.sample_rate
    }

    pub fn duration_to_sample_count(&self, duration: f32) -> usize {
        (duration * self.sample_rate) as usize //TODO: Maybe cache?
    }
}
