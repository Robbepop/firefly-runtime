use crate::color::FromRGB;
use alloc::boxed::Box;
use core::fmt::Display;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X9, MonoTextStyle},
    prelude::*,
    text::Text,
};
use firefly_hal::{Device, DeviceImpl, Duration, Instant};

const FONT_HEIGHT: i32 = 10;
const FONT_WIDTH: i32 = 6;
const BTN_DELAY: Duration = Duration::from_ms(4_000);

/// An alert popup window showing an error message.
pub(crate) struct ErrorScene {
    msg: Box<dyn Display>,
    start: Option<Instant>,
    showed_msg: bool,
    showed_btn: bool,
    enabled_btn: bool,
    buttons: u8,
}

impl ErrorScene {
    pub fn new(msg: Box<dyn Display>) -> Self {
        Self {
            msg,
            start: None,
            showed_msg: false,
            showed_btn: false,
            enabled_btn: false,
            buttons: 0,
        }
    }

    pub fn update(&mut self, device: &mut DeviceImpl) -> bool {
        // Check if the confirmation button is active.
        if !self.enabled_btn {
            let now = device.now();
            let start = match self.start {
                Some(start) => start,
                None => {
                    self.start = Some(now);
                    now
                }
            };
            if now - start > BTN_DELAY {
                self.enabled_btn = true;
            }
        }

        // If the button is active, check if the user pressed and released it.
        if self.enabled_btn {
            let buttons = match device.read_input() {
                Some(input) => input.buttons,
                None => 0u8,
            };
            let buttons = buttons & 0b11111;
            if buttons == 0 {
                if buttons != 0 {
                    self.buttons = buttons;
                } else {
                    return true;
                }
            }
        }
        false
    }

    pub fn render<D, C, E>(&mut self, display: &mut D) -> Result<(), E>
    where
        D: DrawTarget<Color = C, Error = E> + OriginDimensions,
        C: RgbColor + FromRGB,
    {
        if !self.showed_msg {
            let mut text_style = MonoTextStyle::new(&FONT_6X9, C::PRIMARY);
            text_style.background_color = Some(C::BG);
            let point = Point::new(120 - 3 * 13, 71 - FONT_HEIGHT);
            let text = alloc::format!("{}", self.msg);
            let text = Text::new(&text, point, text_style);
            text.draw(display)?;
            self.showed_msg = true;
        }
        if !self.showed_btn {
            let color = if self.enabled_btn {
                C::ACCENT
            } else {
                C::MUTED
            };
            let mut text_style = MonoTextStyle::new(&FONT_6X9, color);
            text_style.background_color = Some(C::BG);
            let point = Point::new(120 - 3 * 13, 120 - FONT_HEIGHT);
            let text = "oh no!";
            let text = Text::new(text, point, text_style);
            text.draw(display)?;
            self.showed_btn = true;
        }
        Ok(())
    }
}
