use std::ops::{Index, IndexMut, AddAssign, Add, Mul, MulAssign};

#[derive(Clone)]
pub struct PolySample(pub Vec<f32>);

impl Index<usize> for PolySample {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for PolySample {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Add<PolySample> for PolySample {
    type Output = PolySample;

    fn add(self, rhs: PolySample) -> Self::Output {
        PolySample(self.0.iter().zip(rhs.0.iter()).map(|(l, r)| l + r).collect())
    }
}

impl AddAssign<PolySample> for PolySample {
    fn add_assign(&mut self, rhs: PolySample) {
        for (l,r) in self.0.iter_mut().zip(rhs.0.iter()) {
            *l+=r;
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

impl Mul<&PolySample> for f32 {
    type Output = PolySample;

    fn mul(self, rhs: &PolySample) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f32> for PolySample {
    fn mul_assign(&mut self, rhs: f32) {
        for l in &mut self.0 {
            *l*=rhs;
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
        self.0 = self.0.repeat(n);
    }
}