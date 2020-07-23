use crate::cpal::Cpal;
use cpal::traits::EventLoopTrait;

pub trait Patch: Send {
    fn next_value(&mut self, sample_timing: &SampleTiming) -> Vec<f32>;
}

#[derive(Clone)]
pub struct SampleTiming {
    pub sample_rate: f32,
    pub clock: usize,
}

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
}

#[derive(Default)]
pub struct MasterPatch {
    patches: Vec<Box<dyn Patch>>,
}

impl MasterPatch {
    pub fn add_patch<P: 'static + Patch>(&mut self, patch: P) {
        self.patches.push(Box::new(patch));
    }

    pub fn play(&mut self, cpal: &mut Cpal, mut sample_timing: SampleTiming) {
        let Cpal {
            event_loop,
            format,
            ..
        } = cpal;

        sample_timing.sample_rate = format.sample_rate.0 as f32;

        let sample_timing_ref = &mut sample_timing;

        event_loop.run(move |id, result| {
            let data = match result {
                Ok(data) => data,
                Err(err) => {
                    eprintln!("an error occurred on stream {:?}: {}", id, err);
                    return;
                }
            };

            match data {
                cpal::StreamData::Output {
                    buffer: cpal::UnknownTypeOutputBuffer::U16(mut buffer),
                } => {
                    for sample in buffer.chunks_mut(format.channels as usize) {
                        let mut next_value_samples =
                            self.next_value(sample_timing_ref).into_iter().cycle();
                        for out in sample.iter_mut() {
                            *out = ((next_value_samples.next().unwrap() * 0.5 + 0.5)
                                * std::u16::MAX as f32) as u16;
                        }
                        sample_timing_ref.tick();
                    }
                }
                cpal::StreamData::Output {
                    buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer),
                } => {
                    for sample in buffer.chunks_mut(format.channels as usize) {
                        let mut next_value_samples =
                            self.next_value(sample_timing_ref).into_iter().cycle();
                        for out in sample.iter_mut() {
                            *out =
                                (next_value_samples.next().unwrap() * std::i16::MAX as f32) as i16;
                        }
                        sample_timing_ref.tick();
                    }
                }
                cpal::StreamData::Output {
                    buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer),
                } => {
                    for sample in buffer.chunks_mut(format.channels as usize) {
                        let mut next_value_samples =
                            self.next_value(sample_timing_ref).into_iter().cycle();
                        for out in sample.iter_mut() {
                            *out = next_value_samples.next().unwrap();
                        }
                        sample_timing_ref.tick();
                    }
                }
                _ => (),
            }
        });
    }
}

impl Patch for MasterPatch {
    fn next_value(&mut self, sample_timing: &SampleTiming) -> Vec<f32> {
        let mut master = Vec::new();
        for patch in &mut self.patches {
            for (i, sample) in patch.next_value(&sample_timing).into_iter().enumerate() {
                match master.get_mut(i) {
                    None => {
                        master.push(sample);
                    }
                    Some(current_sample) => {
                        *current_sample += sample;
                    }
                }
            }
        }
        master
    }
}
