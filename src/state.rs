use crate::Graphics;
use embedded_graphics::geometry::Point;

type C<'a> = wasmi::Caller<'a, State>;

pub struct State {
    graphics: Graphics,
}

impl State {
    pub fn new() -> Self {
        Self {
            graphics: Graphics::new(),
        }
    }

    pub fn link(linker: &mut wasmi::Linker<Self>) -> Result<(), wasmi::errors::LinkerError> {
        linker.func_wrap(
            "graphics",
            "draw_line",
            move |mut caller: C,
                  x1: i32,
                  y1: i32,
                  x2: i32,
                  y2: i32,
                  color: u32,
                  stroke_width: u32| {
                let state = caller.data_mut();
                state.graphics.draw_line(
                    Point::new(x1, y1),
                    Point::new(x2, y2),
                    color as u8,
                    stroke_width,
                );
            },
        )?;
        Ok(())
    }
}
