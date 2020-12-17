use cpal::{traits::{DeviceTrait, HostTrait}, Device, Host, SupportedStreamConfig, StreamConfig};
use crate::patch::OutPatch;
use crate::SampleTiming;
use cpal::traits::StreamTrait;

pub struct Cpal {
    pub host: Host,
    pub device: Device,
    pub config: SupportedStreamConfig,
}

impl Cpal {
    pub fn new() -> Result<Self, anyhow::Error> {
        let host = cpal::default_host();

        let device = host
            .default_output_device()
            .expect("failed to find a default output device");
        let config = device.default_output_config().unwrap();

        Ok(Self {
            host,
            device,
            config,
        })
    }

    pub fn play_patch<P: 'static + OutPatch>(&self, patch: P) {
        match self.config.sample_format() {
            cpal::SampleFormat::F32 => self.play_on::<f32, P>(patch).unwrap(),
            cpal::SampleFormat::I16 => self.play_on::<i16, P>(patch).unwrap(),
            cpal::SampleFormat::U16 => self.play_on::<u16, P>(patch).unwrap(),
        }
    }

    fn play_on<T, P: 'static + OutPatch>(&self, mut patch: P) -> Result<(), anyhow::Error>
        where
            T: cpal::Sample,
    {
        let config: &StreamConfig = &self.config.clone().into();

        let mut sample_timing = SampleTiming::new(config.sample_rate.0 as f32);
        let channels = config.channels as usize;

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let stream = self.device.build_output_stream(
            config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                patch.write_data(data, channels, &mut sample_timing)
            },
            err_fn,
        )?;
        stream.play()?;
        std::thread::sleep(std::time::Duration::from_millis(1000));

        Ok(())
    }
}
