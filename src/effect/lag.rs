use super::Effect;
use crate::{PolySample, SampleTiming};
use rand::Rng;
use std::collections::VecDeque;

#[derive(Clone, Default)]
pub struct Lag {
    start_chance: f32,
    stop_chance: f32,
    buffer_length: f32,
    buffer_length_rand_bias: f32,
    active_indexed_buffer: Option<(usize, Vec<PolySample>)>,
    buffer: VecDeque<PolySample>,
}

impl Lag {
    pub fn new(
        start_chance: f32,
        stop_chance: f32,
        buffer_length: f32,
        buffer_length_rand_bias: f32,
    ) -> Self {
        debug_assert!(buffer_length >= buffer_length_rand_bias);
        Self {
            start_chance,
            stop_chance,
            buffer_length,
            buffer_length_rand_bias,
            active_indexed_buffer: None,
            buffer: VecDeque::new(),
        }
    }
}

impl Effect for Lag {
    fn process(&mut self, sample_timing: &SampleTiming, poly_sample: PolySample) -> PolySample {
        let mut output = poly_sample;

        if let Some((ref mut index, ref mut buffer)) = self.active_indexed_buffer {
            output = buffer[*index].clone();
            if rand::thread_rng().gen::<f32>() < self.stop_chance {
                self.active_indexed_buffer = None;
            } else {
                *index += 1;
                if *index == buffer.len() {
                    *index = 0;
                }
            }
        } else {
            let mut rng = rand::thread_rng();
            if rng.gen::<f32>() < self.start_chance {
                let active_buffer_length =
                    self.buffer_length + rng.gen_range(-1.0, 1.0) * self.buffer_length_rand_bias;
                let active_buffer_size =
                    sample_timing.duration_to_sample_count(active_buffer_length);
                let buffer_size = self.buffer.len();
                if buffer_size >= active_buffer_size {
                    self.active_indexed_buffer = Some((
                        0,
                        self.buffer.make_contiguous()[buffer_size - active_buffer_size..].to_vec(),
                    ));
                }
            }
        }

        let buffer_size = sample_timing
            .duration_to_sample_count(self.buffer_length + self.buffer_length_rand_bias);
        if self.buffer.len() == buffer_size {
            self.buffer.pop_front();
        }
        self.buffer.push_back(output.clone());
        output
    }
}
