use cpal::{
    traits::{DeviceTrait, EventLoopTrait, HostTrait},
    Device, EventLoop, Format, Host, StreamId,
};

pub struct Cpal {
    pub host: Host,
    pub device: Device,
    pub format: Format,
    pub event_loop: EventLoop,
    pub stream_id: StreamId,
}

impl Cpal {
    pub fn new() -> Result<Self, anyhow::Error> {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("failed to find a default output device");
        let format = device.default_output_format()?;
        let event_loop = host.event_loop();
        let stream_id = event_loop.build_output_stream(&device, &format)?;
        event_loop.play_stream(stream_id.clone())?;

        Ok(Self {
            host,
            device,
            format,
            event_loop,
            stream_id,
        })
    }
}
