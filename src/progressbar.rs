use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::io::{stdout, Write};

pub struct ProgressBar {
    pub value: u16,
    pub max: u16,
    pub width: u16,
    pub label: String,
}

impl ProgressBar {
    pub fn new(value: u16, max: u16, width: u16, label: String) -> Self {
        Self {
            value,
            max,
            width,
            label,
        }
    }

    pub fn render(&self) -> Result<(), Box<dyn std::error::Error>> {
        let percent = (self.value as f64 / self.max as f64) * 100.0;
        let filled_width = (percent / 100.0 * self.width as f64).round() as u16;
        let mut stdout = stdout();

        enable_raw_mode()?;

        execute!(stdout, cursor::MoveToColumn(0))?;
        execute!(stdout, Clear(ClearType::UntilNewLine))?;
        execute!(stdout, Print(format!("{: <20}", self.label)))?;
        execute!(stdout, Print("["))?;
        execute!(stdout, SetForegroundColor(Color::Green))?;

        for _ in 0..filled_width {
            execute!(stdout, Print("■"))?;
        }

        execute!(stdout, ResetColor)?;
        for _ in filled_width..self.width {
            execute!(stdout, Print("-"))?;
        }

        execute!(
            stdout,
            Print("] "),
            Print(format!("{:.1}%", percent)),
            cursor::MoveToNextLine(1)
        )?;

        disable_raw_mode()?;
        stdout.flush()?;
        Ok(())
    }
}
