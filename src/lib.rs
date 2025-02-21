use crossterm::style::Color;

pub mod infobox;
pub mod listselector;
pub mod progressbar;
pub mod prompt;
pub mod spinner;

#[cfg(feature = "full-tui-interface")]
pub mod meter;

pub mod color {
    pub use crossterm::style::Color;
}

// Theme Support (new for 0.3.0)

/// A struct representing a global theme for all of your projects widgets.
pub struct GlobalTheme {
    pub primary: Color,
    pub secondary: Color,
    pub tertiary: Color,

    pub style: String, //
}