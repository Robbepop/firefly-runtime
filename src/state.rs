use crate::config::FullID;
use crate::frame_buffer::FrameBuffer;
use firefly_device::{Device, DeviceImpl};
use firefly_meta::ValidationError;

pub(crate) struct State {
    pub device: DeviceImpl,
    pub id:     FullID,
    pub frame:  FrameBuffer,
    pub seed:   u32,
    pub memory: Option<wasmi::Memory>,
    pub exit:   bool,
    pub next:   Option<FullID>,
}

impl State {
    pub(crate) fn new(id: FullID, device: DeviceImpl) -> Self {
        Self {
            device,
            id,
            frame: FrameBuffer::new(),
            seed: 0,
            memory: None,
            next: None,
            exit: false,
        }
    }

    pub(crate) fn log_validation_error(&self, source: &str, msg: &str, err: ValidationError) {
        self.device.log_error(source, msg);
        self.device.log_error(source, err.as_str());
    }
}
