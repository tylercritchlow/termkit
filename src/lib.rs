use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;

pub mod infobox;
pub mod listselector;
pub mod progressbar;
pub mod prompt;
pub mod spinner;
pub mod table;

pub mod color {
    pub use crossterm::style::Color;
}

pub(crate) struct RawModeGuard;

impl RawModeGuard {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        enable_raw_mode()?;
        Ok(Self)
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
    }
}
