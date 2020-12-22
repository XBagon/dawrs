use crate::prelude::*;

pub trait Patch: Send {
    fn next_sample(&mut self, sample_timing: &SampleTiming) -> PolySample;
}

pub trait OutPatch: Patch {
    fn write_data<T: cpal::Sample>(
        &mut self,
        output: &mut [T],
        channels: usize,
        sample_timing: &mut SampleTiming,
    );
}

#[derive(Default)]
pub struct MasterPatch {
    patches: Vec<Box<dyn Patch>>,
}

impl MasterPatch {
    pub fn new() -> Self {
        MasterPatch {
            patches: Vec::new(),
        }
    }

    pub fn add_patch<P: 'static + Patch>(&mut self, patch: P) {
        self.patches.push(Box::new(patch));
    }
}

impl Patch for MasterPatch {
    fn next_sample(&mut self, sample_timing: &SampleTiming) -> PolySample {
        let mut master = poly_sample!();
        for patch in &mut self.patches {
            for (i, sample) in patch.next_sample(&sample_timing).0.into_iter().enumerate() {
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

impl OutPatch for MasterPatch {
    fn write_data<T: cpal::Sample>(
        &mut self,
        output: &mut [T],
        channels: usize,
        sample_timing: &mut SampleTiming,
    ) {
        for frame in output.chunks_mut(channels) {
            let mut next_samples = self
                .next_sample(sample_timing)
                .0
                .into_iter()
                .chain(std::iter::repeat(0.0))
                .map(|s| cpal::Sample::from(&s));
            for sample in frame.iter_mut() {
                *sample = next_samples.next().unwrap();
            }
            sample_timing.tick();
        }
    }
}
