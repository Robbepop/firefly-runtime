pub struct Device<D, C, T, S>
where
    D: embedded_graphics::draw_target::DrawTarget<Color = C>,
    C: embedded_graphics::pixelcolor::RgbColor,
    T: Timer,
    S: embedded_storage::Storage,
{
    pub display: D,
    pub timer: T,
    pub storage: S,
}

pub trait Timer {
    fn sleep(ms: u32);
    fn uptime() -> u32;
}
