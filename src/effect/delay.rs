use super::Effect;
use crate::SampleTiming;
use std::collections::VecDeque;

#[derive(Clone, Default)]
pub struct Delay {
    delay: f32,
    feedback: f32,
    buffer: VecDeque<Vec<f32>>,
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
    fn process(&mut self, sample_timing: &SampleTiming, mut sample: Vec<f32>) -> Vec<f32> {
        let buffer_size = sample_timing.duration_to_sample_count(self.delay);
        if self.buffer.len() == buffer_size {
            for (sample_value, buffered_sample_value) in
                sample.iter_mut().zip(self.buffer[0].iter())
            {
                *sample_value += buffered_sample_value * self.feedback
            }
            self.buffer.pop_front();
        }
        self.buffer.push_back(sample.clone());
        sample
    }
}
