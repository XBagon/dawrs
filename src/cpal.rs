use crate::{patch::OutPatch, SampleTiming};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, Host, Stream, StreamConfig, SupportedStreamConfig,
};
use std::{mem::ManuallyDrop, ptr, sync::mpsc};

pub struct Cpal<P: OutPatch + 'static> {
    pub host: Host,
    pub device: Device,
    pub config: SupportedStreamConfig,
    pub active_stream: Option<Stream>,
    pub receiver: Option<mpsc::Receiver<P>>,
}

struct ReturningPatch<P: OutPatch> {
    patch: ManuallyDrop<P>,
    sender: mpsc::Sender<P>,
}

impl<P: OutPatch> Drop for ReturningPatch<P> {
    fn drop(&mut self) {
        let patch = unsafe { ManuallyDrop::into_inner(ptr::read(&self.patch)) };
        self.sender.send(patch).unwrap();
    }
}

impl<P: OutPatch> Cpal<P> {
    pub fn new() -> Result<Self, anyhow::Error> {
        let host = cpal::default_host();

        let device = host.default_output_device().expect("failed to find a default output device");
        let config = device.default_output_config().unwrap();

        Ok(Self {
            host,
            device,
            config,
            active_stream: None,
            receiver: None,
        })
    }

    pub fn play_patch(&mut self, patch: P) {
        match self.config.sample_format() {
            cpal::SampleFormat::F32 => self.play_on::<f32>(patch).unwrap(),
            cpal::SampleFormat::I16 => self.play_on::<i16>(patch).unwrap(),
            cpal::SampleFormat::U16 => self.play_on::<u16>(patch).unwrap(),
        }
    }

    fn play_on<T>(&mut self, patch: P) -> Result<(), anyhow::Error>
    where
        T: cpal::Sample,
    {
        let config: &StreamConfig = &self.config.clone().into();

        let mut sample_timing = SampleTiming::new(config.sample_rate.0 as f32);
        let channels = config.channels as usize;

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let (sender, receiver) = mpsc::channel();
        self.receiver = Some(receiver);
        let mut returning_patch = ReturningPatch {
            patch: ManuallyDrop::new(patch),
            sender,
        };

        let stream = self.device.build_output_stream(
            config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                returning_patch.patch.write_data(data, channels, &mut sample_timing)
            },
            err_fn,
        )?;
        stream.play()?;
        self.active_stream = Some(stream);

        Ok(())
    }
}
