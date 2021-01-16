use std::ops::{Add, AddAssign, Sub, SubAssign};

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
    pub fn sample_clock(&self) -> f32 {
        self.clock as f32 / self.sample_rate
    }

    pub fn sample_clock_with_frequency(&self, frequency: f32) -> f32 {
        let sample_clock_length = (1.0 / frequency * self.sample_rate * frequency.ceil()) as usize; //TODO: Maybe cache?
        ((self.clock + sample_clock_length) % sample_clock_length) as f32 / self.sample_rate
    }

    pub fn sample_clock_with_length(&self, length: f32) -> f32 {
        let sample_clock_length = (length * self.sample_rate * (1.0 / length).ceil()) as usize; //TODO: Maybe cache?
        ((self.clock + sample_clock_length) % sample_clock_length) as f32 / self.sample_rate
    }

    pub fn duration_to_sample_count(&self, duration: f32) -> usize {
        (duration * self.sample_rate) as usize //TODO: Maybe cache?
    }
}

//ops for SampleTiming

impl Add for SampleTiming {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            sample_rate: self.sample_rate,
            clock: self.clock + other.clock,
        }
    }
}

impl AddAssign for SampleTiming {
    fn add_assign(&mut self, other: Self) {
        self.clock += other.clock
    }
}

impl Sub for SampleTiming {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            sample_rate: self.sample_rate,
            clock: self.clock - other.clock,
        }
    }
}

impl SubAssign for SampleTiming {
    fn sub_assign(&mut self, other: Self) {
        self.clock -= other.clock
    }
}

impl Add<usize> for SampleTiming {
    type Output = Self;

    fn add(self, other: usize) -> Self::Output {
        Self {
            sample_rate: self.sample_rate,
            clock: self.clock + other,
        }
    }
}

impl AddAssign<usize> for SampleTiming {
    fn add_assign(&mut self, other: usize) {
        self.clock += other
    }
}

impl Sub<usize> for SampleTiming {
    type Output = Self;

    fn sub(self, other: usize) -> Self::Output {
        Self {
            sample_rate: self.sample_rate,
            clock: self.clock - other,
        }
    }
}

impl SubAssign<usize> for SampleTiming {
    fn sub_assign(&mut self, other: usize) {
        self.clock -= other
    }
}

impl Add<SampleTiming> for usize {
    type Output = SampleTiming;

    fn add(self, other: SampleTiming) -> Self::Output {
        Self::Output {
            sample_rate: other.sample_rate,
            clock: self + other.clock,
        }
    }
}

impl Sub<SampleTiming> for usize {
    type Output = SampleTiming;

    fn sub(self, other: SampleTiming) -> Self::Output {
        Self::Output {
            sample_rate: other.sample_rate,
            clock: self - other.clock,
        }
    }
}

//Ops for &SampleTiming

impl Add for &SampleTiming {
    type Output = SampleTiming;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            sample_rate: self.sample_rate,
            clock: self.clock + other.clock,
        }
    }
}

impl AddAssign for &mut SampleTiming {
    fn add_assign(&mut self, other: Self) {
        self.clock += other.clock
    }
}

impl Sub for &SampleTiming {
    type Output = SampleTiming;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            sample_rate: self.sample_rate,
            clock: self.clock - other.clock,
        }
    }
}

impl SubAssign for &mut SampleTiming {
    fn sub_assign(&mut self, other: Self) {
        self.clock -= other.clock
    }
}

impl Add<usize> for &SampleTiming {
    type Output = SampleTiming;

    fn add(self, other: usize) -> Self::Output {
        Self::Output {
            sample_rate: self.sample_rate,
            clock: self.clock + other,
        }
    }
}

impl AddAssign<usize> for &mut SampleTiming {
    fn add_assign(&mut self, other: usize) {
        self.clock += other
    }
}

impl Sub<usize> for &SampleTiming {
    type Output = SampleTiming;

    fn sub(self, other: usize) -> Self::Output {
        Self::Output {
            sample_rate: self.sample_rate,
            clock: self.clock - other,
        }
    }
}

impl SubAssign<usize> for &mut SampleTiming {
    fn sub_assign(&mut self, other: usize) {
        self.clock -= other
    }
}

impl Add<&SampleTiming> for usize {
    type Output = SampleTiming;

    fn add(self, other: &SampleTiming) -> Self::Output {
        Self::Output {
            sample_rate: other.sample_rate,
            clock: self + other.clock,
        }
    }
}

impl Sub<&SampleTiming> for usize {
    type Output = SampleTiming;

    fn sub(self, other: &SampleTiming) -> Self::Output {
        Self::Output {
            sample_rate: other.sample_rate,
            clock: self - other.clock,
        }
    }
}
