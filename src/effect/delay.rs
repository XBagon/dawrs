use super::Effect;
use crate::{PolySample, SampleTiming};
use std::collections::VecDeque;

#[derive(Clone, Default)]
pub struct Delay {
    pub delay: f32,
    pub feedback: f32,
    buffer: VecDeque<PolySample>,
}

impl Delay {
    pub fn new(delay: f32, feedback: f32) -> Self {
        Self {
            delay,
            feedback,
            buffer: VecDeque::new(),
        }
    }
}

impl Effect for Delay {
    fn process(&mut self, sample_timing: &SampleTiming, mut poly_sample: PolySample) -> PolySample {
        let buffer_size = sample_timing.duration_to_sample_count(self.delay);
        if self.buffer.len() == buffer_size {
            poly_sample += self.buffer.pop_front().unwrap();
        }
        self.buffer.push_back(poly_sample.clone() * self.feedback);
        poly_sample
    }
}
