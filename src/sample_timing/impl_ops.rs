use std::ops::{Add, AddAssign, Sub, SubAssign};
use super::SampleTiming;

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