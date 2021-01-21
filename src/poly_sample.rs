use shrinkwraprs::Shrinkwrap;
use std::ops::{Add, AddAssign, Mul, MulAssign, Range};

#[cfg(feature = "mono")]
#[derive(Clone, Debug, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct PolySample(pub tinyvec::ArrayVec<[f32; 1]>);

#[cfg(feature = "stereo")]
#[derive(Clone, Debug, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct PolySample(pub tinyvec::ArrayVec<[f32; 2]>);

#[cfg(feature = "unlimited")]
#[derive(Clone, Debug, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct PolySample(pub tinyvec::TinyVec<[f32; 2]>);

#[cfg(feature = "unlimited")]
#[macro_export]
macro_rules! poly_sample {
    ([$e:expr; $n:expr]) => (PolySample(tinyvec::tiny_vec![$e; $n]));
    ([$($e:expr)*]) => (PolySample(tinyvec::tiny_vec![$($e)*]));
    ([$e:expr]) => (PolySample(tinyvec::tiny_vec![_ => $e]));
    ($e:expr) => (PolySample($e));
    () => (PolySample(tinyvec::tiny_vec![]));
}

#[cfg(not(feature = "unlimited"))]
#[macro_export]
macro_rules! poly_sample {
    ([$e:expr; $n:expr]) => (PolySample(tinyvec::array_vec![$e; $n]));
    ([$($e:expr)*]) => (PolySample(tinyvec::array_vec![$($e)*]));
    ([$e:expr]) => (PolySample(tinyvec::array_vec![_ => $e]));
    ($e:expr) => (PolySample($e));
    () => (PolySample(tinyvec::array_vec![]));
}

impl Add<PolySample> for PolySample {
    type Output = PolySample;

    fn add(self, rhs: PolySample) -> Self::Output {
        PolySample(self.0.iter().zip(rhs.0.iter()).map(|(l, r)| l + r).collect())
    }
}

impl AddAssign<PolySample> for PolySample {
    fn add_assign(&mut self, rhs: PolySample) {
        for (l, r) in self.0.iter_mut().zip(rhs.0.iter()) {
            *l += r;
        }
    }
}

impl Mul<f32> for PolySample {
    type Output = PolySample;

    fn mul(self, rhs: f32) -> Self::Output {
        PolySample(self.0.iter().map(|l| l * rhs).collect())
    }
}

impl Mul<f32> for &PolySample {
    type Output = PolySample;

    fn mul(self, rhs: f32) -> Self::Output {
        PolySample(self.0.iter().map(|l| l * rhs).collect())
    }
}

impl Mul<PolySample> for f32 {
    type Output = PolySample;

    fn mul(self, rhs: PolySample) -> Self::Output {
        rhs * self
    }
}

impl Mul<&PolySample> for f32 {
    type Output = PolySample;

    fn mul(self, rhs: &PolySample) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f32> for PolySample {
    fn mul_assign(&mut self, rhs: f32) {
        for l in &mut self.0 {
            *l *= rhs;
        }
    }
}

impl PolySample {
    /// Multiplies `self` by cycled `other`.
    /// Especially useful for volume effects.
    pub fn apply(&mut self, other: &Self) {
        for (l, r) in &mut self.0.iter_mut().zip(other.0.iter().cycle()) {
            *l *= r;
        }
    }

    /// Clones all channels `n` times and concatenates them.
    pub fn polify(&mut self, n: usize) {
        for _ in 0..(n - 1) {
            self.0.append(&mut self.0.clone());
        }
    }

    pub fn linear_map(&mut self, from: Range<f32>, to: Range<f32>) {
        for sample in &mut self.0 {
            *sample = (*sample + (from.start - to.start))
                * ((to.end - to.start) / (from.end - from.start))
        }
    }
}
