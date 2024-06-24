use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
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

        // Calculate the total width of the progress bar, percentage text, and label padding
        let total_width = self.width + 9 + (20 - self.label.len() as u16);

        // Move to the beginning of the line (column 0)
        execute!(stdout, cursor::MoveToColumn(0))?;

        // Clear the entire line
        execute!(stdout, Clear(ClearType::UntilNewLine))?;

        // Print the label, ensuring it takes up 20 characters with padding
        execute!(stdout, Print(format!("{: <20}", self.label)))?;

        // Start the progress bar with "["
        execute!(stdout, Print("["))?;

        // Print the filled portion in green
        execute!(stdout, SetForegroundColor(Color::Green))?;
        for _ in 0..filled_width {
            execute!(stdout, Print("■"))?;
        }

        // Reset the color and print the remaining portion
        execute!(stdout, ResetColor)?;
        for _ in filled_width..self.width {
            execute!(stdout, Print("-"))?;
        }

        // Close the progress bar, print the percentage, and move to the next line
        execute!(
            stdout,
            Print("] "),
            Print(format!("{:.1}%", percent)),
            cursor::MoveToNextLine(1)
        )?;

        stdout.flush()?;
        Ok(())
    }
}
