use crate::{patch::OutPatch, SampleTiming};
use anyhow::Result;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, Host, StreamConfig, SupportedStreamConfig,
};
use std::{marker::PhantomData, mem::ManuallyDrop, ptr};

pub struct Cpal<P: OutPatch + 'static> {
    pub host: Host,
    pub device: Device,
    pub config: SupportedStreamConfig,
    phantom: PhantomData<P>,
}

struct CpalPatch<P: OutPatch> {
    patch: ManuallyDrop<P>,
    return_sender: crossbeam_channel::Sender<P>,
    event_sender: crossbeam_channel::Sender<CpalEvent>,
}

impl<P: OutPatch> Drop for CpalPatch<P> {
    fn drop(&mut self) {
        let patch = unsafe { ManuallyDrop::into_inner(ptr::read(&self.patch)) };
        self.return_sender.send(patch).unwrap();
    }
}

pub enum CpalEvent {
    Exit,
    Pause,
    Resume,
}

impl<P: OutPatch> Cpal<P> {
    pub fn new() -> Result<Self> {
        let host = cpal::default_host();

        let device = host.default_output_device().expect("failed to find a default output device");
        let config = device.default_output_config()?;

        Ok(Self {
            host,
            device,
            config,
            phantom: PhantomData::default(),
        })
    }

    pub fn play_patch(&mut self, patch: &mut P) {
        match self.config.sample_format() {
            cpal::SampleFormat::F32 => self.play_on::<f32>(patch).unwrap(),
            cpal::SampleFormat::I16 => self.play_on::<i16>(patch).unwrap(),
            cpal::SampleFormat::U16 => self.play_on::<u16>(patch).unwrap(),
        }
    }

    fn play_on<T>(&mut self, patch: &mut P) -> Result<()>
    where
        T: cpal::Sample,
    {
        let config: &StreamConfig = &self.config.clone().into();

        let mut sample_timing = SampleTiming::new(config.sample_rate.0 as f32);
        let channels = config.channels as usize;

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let (return_sender, return_receiver) = crossbeam_channel::bounded(1);
        let (event_sender, event_receiver) = crossbeam_channel::bounded(1);

        let mut result = Ok(());

        take_mut::take(patch, |patch| {
            let mut cpal_patch = CpalPatch {
                patch: ManuallyDrop::new(patch),
                return_sender,
                event_sender,
            };

            (|| -> Result<P> {
                let stream = self.device.build_output_stream(
                    config,
                    move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                        if let Some(event) =
                            cpal_patch.patch.write_data(data, channels, &mut sample_timing)
                        {
                            cpal_patch.event_sender.send(event).unwrap();
                        }
                    },
                    err_fn,
                )?;
                stream.play().unwrap();
                loop {
                    match event_receiver.recv().unwrap() {
                        CpalEvent::Exit => {
                            stream.pause().unwrap();
                            drop(stream);
                            return Ok(return_receiver.recv().unwrap());
                        }
                        CpalEvent::Pause => {todo!()}
                        CpalEvent::Resume => {todo!()}
                    }
                }
            })()
            .unwrap_or_else(|err| {
                result = Err(err);
                return_receiver.recv().unwrap()
            })
        });
        Ok(())
    }
}
