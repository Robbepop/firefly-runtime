use crate::color::FromRGB;
use core::marker::PhantomData;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::OriginDimensions;
use embedded_graphics::pixelcolor::RgbColor;

pub struct Device<D, C, T, S, R>
where
    D: DrawTarget<Color = C> + OriginDimensions,
    C: RgbColor + FromRGB,
    T: Timer,
    S: Storage<R>,
    R: embedded_io::Read,
{
    pub display: D,
    pub timer:   T,
    pub storage: S,
    pub reader:  PhantomData<R>,
}

pub trait Timer {
    /// Pause the game execution (in ms).
    fn sleep(&self, ms: u64);

    /// Time passed since the last reboot (in ms).
    fn uptime(&self) -> u64;
}

/// File system abstraction.
///
/// Designed to work nicely with [embedded_sdmmc] and the stdlib filesystem.
///
/// embedded_sdmmc: https://github.com/rust-embedded-community/embedded-sdmmc-rs
pub trait Storage<R: embedded_io::Read> {
    /// Open a file for reading.
    ///
    /// The file path is given as a slice of path components.
    /// There are at least 2 components: the first one is the root directory
    /// (either "roms" or "data"), the last one is the file name,
    /// and everything in between are directory names if the file is nested.
    fn open_file(&self, path: &[&str]) -> R;
}
