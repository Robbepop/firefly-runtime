use embedded_graphics::framebuffer::{buffer_size, Framebuffer};
use embedded_graphics::pixelcolor::raw::{LittleEndian, RawU2};
use embedded_graphics::pixelcolor::Gray2;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::*;

const WIDTH: usize = 320;
const HEIGHT: usize = 240;
const BUFFER_SIZE: usize = buffer_size::<Gray2>(WIDTH, HEIGHT);

pub(crate) struct Graphics {
    frame: Framebuffer<Gray2, RawU2, LittleEndian, WIDTH, HEIGHT, BUFFER_SIZE>,
}

impl Graphics {
    pub(crate) fn new() -> Self {
        Self {
            frame: Framebuffer::new(),
        }
    }

    pub(crate) fn draw_line(&mut self, start: Point, end: Point, color: u8, stroke_width: u32) {
        let line = Line::new(start, end);
        let color = Gray2::new(color);
        let style = PrimitiveStyle::with_stroke(color, stroke_width);
        log_error(line.draw_styled(&style, &mut self.frame));
    }
}

fn log_error<T, E: core::fmt::Debug>(res: Result<T, E>) {
    if let Err(err) = res {
        panic!("{err:?}")
    }
}
