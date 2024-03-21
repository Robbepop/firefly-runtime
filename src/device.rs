pub struct Device<Display, Color, Delay, Storage>
where
    Display: embedded_graphics::draw_target::DrawTarget<Color = Color>,
    Color: embedded_graphics::pixelcolor::RgbColor,
    Delay: Fn(u32),
    Storage: embedded_storage::Storage,
{
    pub display:  Display,
    pub delay_ms: Delay,
    pub storage:  Storage,
}
