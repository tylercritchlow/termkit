use crossterm::{
    cursor, execute,
    style::Print,
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Write};
use textwrap::fill;

pub struct InfoBox {
    pub title: String,
    pub message: String,
    pub width: usize,
    pub padding: usize,  // New field for padding
}

impl InfoBox {
    pub fn new(title: String, message: String, width: usize, padding: Option<usize>) -> Self {
        Self {
            title,
            message,
            width,
            padding: padding.unwrap_or(2), // Default to 2 if padding is not provided
        }
    }

    pub fn render(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut stdout = stdout();
        let total_width = self.width + 2 * self.padding + 2; // Include padding

        execute!(stdout, cursor::MoveToColumn(0))?;
        execute!(stdout, Clear(ClearType::UntilNewLine))?;
        execute!(
            stdout,
            Print(format!("{: <width$}", self.title, width = total_width as usize))
        )?;
        execute!(stdout, Print("\n"))?;
        execute!(stdout, Print("┌"))?;
        for _ in 0..total_width - 2 {
            execute!(stdout, Print("─"))?;
        }
        execute!(stdout, Print("┐\n"))?;

        let wrapped_message = fill(&self.message, self.width);
        for line in wrapped_message.lines() {
            execute!(stdout, Print("│"))?;
            for _ in 0..self.padding {
                execute!(stdout, Print(" "))?;
            }
            execute!(stdout, Print(line))?;
            for _ in 0..self.padding + (self.width - line.len()) {
                execute!(stdout, Print(" "))?;
            }
            execute!(stdout, Print("│\n"))?;
        }

        execute!(stdout, Print("└"))?;
        for _ in 0..total_width - 2 {
            execute!(stdout, Print("─"))?;
        }
        execute!(stdout, Print("┘\n"))?;
        stdout.flush()?;

        Ok(())
    }
}
