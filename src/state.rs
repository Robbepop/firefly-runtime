use crate::frame_buffer::FrameBuffer;
use core::str::FromStr;
use firefly_device::{Device, DeviceImpl};
use firefly_meta::ValidationError;
use heapless::String;

pub enum Transition {
    /// Continue execution of the current app.
    Continue,
    /// Replace the current app with a new one.
    Replace(String<16>, String<16>),
    // Exit,
}

pub(crate) struct State {
    pub device:    DeviceImpl,
    pub author_id: String<16>,
    pub app_id:    String<16>,
    pub frame:     FrameBuffer,
    pub seed:      u32,
    pub memory:    Option<wasmi::Memory>,
    pub next:      Transition,
}

impl State {
    pub(crate) fn new(author_id: &str, app_id: &str, device: DeviceImpl) -> Self {
        Self {
            device,
            author_id: String::from_str(author_id).unwrap(),
            app_id: String::from_str(app_id).unwrap(),
            frame: FrameBuffer::new(),
            seed: 0,
            memory: None,
            next: Transition::Continue,
        }
    }

    pub(crate) fn log_validation_error(&self, source: &str, msg: &str, err: ValidationError) {
        self.device.log_error(source, msg);
        self.device.log_error(source, err.as_str());
    }
}
