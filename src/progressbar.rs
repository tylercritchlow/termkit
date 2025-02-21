use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{size, Clear, ClearType},
};
use std::io::{stdout, Write};

pub struct ProgressBar {
    value: u16,
    max: u16,
    width: u16,
    label: String,
}

impl ProgressBar {
    pub fn new(label: &str, max: u16, width: u16) -> Self {
        Self {
            value: 0,
            max,
            width,
            label: label.to_string(),
        }
    }

    pub fn update(&mut self, value: u16) {
        self.value = value.min(self.max);
    }

    pub fn render(&self) -> Result<(), Box<dyn std::error::Error>> {
        let percent = (self.value as f64 / self.max as f64) * 100.0;
        let filled_width = (percent / 100.0 * self.width as f64).round() as u16;
        let mut stdout = stdout();

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

        execute!(stdout, Print("] "), Print(format!("{:.1}%", percent)),)?;

        stdout.flush()?;
        Ok(())
    }
}

pub struct ProgressBarManager {
    bars: Vec<ProgressBar>,
}

impl ProgressBarManager {
    pub fn new() -> Self {
        Self { bars: Vec::new() }
    }

    pub fn add_bar(&mut self, label: &str, max: u16, width: u16) {
        self.bars.push(ProgressBar::new(label, max, width));
    }

    pub fn update_bar(&mut self, index: usize, value: u16) {
        if let Some(bar) = self.bars.get_mut(index) {
            bar.update(value);
        }
    }

    pub fn render_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut stdout = stdout();

        let (cursor_x, cursor_y) = cursor::position()?;
        let (_, terminal_height) = size()?;
        let space_below = terminal_height.saturating_sub(cursor_y + 1);
        let start_row = if space_below >= self.bars.len() as u16 {
            cursor_y + 1
        } else {
            cursor_y.saturating_sub(self.bars.len() as u16)
        };

        for (i, bar) in self.bars.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(0, start_row + i as u16))?;
            bar.render()?;
        }

        execute!(stdout, cursor::MoveTo(cursor_x, cursor_y))?;

        stdout.flush()?;
        Ok(())
    }
}
