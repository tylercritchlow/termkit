use crossterm::{
    cursor, execute,
    style::{Color, Print, PrintStyledContent, Stylize},
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Write};
use textwrap::fill;

pub struct InfoBox {
    pub title: String,
    pub message: String,
    pub width: usize,
    pub padding: usize,
    pub title_color: Color,
    pub border_color: Color,
    pub message_color: Color,
}

impl InfoBox {
    pub fn new(
        title: String,
        message: String,
        width: usize,
        title_color: Option<Color>,
        border_color: Option<Color>,
        message_color: Option<Color>,
    ) -> Self {
        Self {
            title,
            message,
            width,
            padding: 2,
            title_color: title_color.unwrap_or(Color::White), // Default to White if not provided
            border_color: border_color.unwrap_or(Color::Blue),
            message_color: message_color.unwrap_or(Color::Reset),
        }
    }

    pub fn with_padding(mut self, padding: usize) -> Self {
        self.padding = padding;
        self
    }

    pub fn render(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut stdout = stdout();
        let total_width = self.width + 2 * self.padding + 2;

        execute!(stdout, cursor::MoveToColumn(0))?;
        execute!(stdout, Clear(ClearType::UntilNewLine))?;

        execute!(
            stdout,
            PrintStyledContent(
                format!("{: <width$}", self.title, width = total_width as usize)
                    .with(self.title_color)
            )
        )?;
        execute!(stdout, Print("\n"))?;

        execute!(stdout, PrintStyledContent("┌".with(self.border_color)))?;
        for _ in 0..total_width - 2 {
            execute!(stdout, PrintStyledContent("─".with(self.border_color)))?;
        }
        execute!(stdout, PrintStyledContent("┐\n".with(self.border_color)))?;

        let wrapped_message = fill(&self.message, self.width);
        for line in wrapped_message.lines() {
            execute!(stdout, PrintStyledContent("│".with(self.border_color)))?;
            for _ in 0..self.padding {
                execute!(stdout, Print(" "))?;
            }
            execute!(stdout, PrintStyledContent(line.with(self.message_color)))?;
            for _ in 0..self.padding + (self.width - line.len()) {
                execute!(stdout, Print(" "))?;
            }
            execute!(stdout, PrintStyledContent("│\n".with(self.border_color)))?;
        }

        execute!(stdout, PrintStyledContent("└".with(self.border_color)))?;
        for _ in 0..total_width - 2 {
            execute!(stdout, PrintStyledContent("─".with(self.border_color)))?;
        }
        execute!(stdout, PrintStyledContent("┘\n".with(self.border_color)))?;

        stdout.flush()?;

        Ok(())
    }
}
