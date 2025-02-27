use crossterm::{
    cursor, execute,
    style::{Color, Print, PrintStyledContent, Stylize},
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Stdout, Write};
use textwrap::fill;

pub struct InfoBox<W: Write = Stdout> {
    pub title: String,
    pub message: String,
    pub width: usize,
    pub padding: usize,
    pub title_color: Color,
    pub border_color: Color,
    pub message_color: Color,
    pub writer: W,
} 

impl InfoBox<Stdout> {
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
            title_color: title_color.unwrap_or(Color::White),
            border_color: border_color.unwrap_or(Color::Blue),
            message_color: message_color.unwrap_or(Color::Reset),
            writer: stdout(),
        }
    }
}

impl<W: Write> InfoBox<W> {
    pub fn with_padding(mut self, padding: usize) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_custom_writer<W2: Write>(self, writer: W2) -> InfoBox<W2> {
        InfoBox {
            title: self.title,
            message: self.message,
            width: self.width,
            padding: self.padding,
            title_color: self.title_color,
            border_color: self.border_color,
            message_color: self.message_color,
            writer,
        }
    }

    pub fn render(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let t   otal_width = self.width + 2 * self.padding + 2;
        execute!(self.writer, cursor::MoveToColumn(0))?;
        execute!(self.writer, Clear(ClearType::UntilNewLine))?;

        execute!(
            self.writer,
            PrintStyledContent(
                format!("{: <width$}", self.title, width = total_width)
                    .with(self.title_color)
            )
        )?;
        execute!(self.writer, Print("\n"))?;

        execute!(self.writer, PrintStyledContent("┌".with(self.border_color)))?;
        for _ in 0..total_width - 2 {
            execute!(self.writer, PrintStyledContent("─".with(self.border_color)))?;
        }
        execute!(self.writer, PrintStyledContent("┐\n".with(self.border_color)))?;

        let wrapped_message = fill(&self.message, self.width);
        for line in wrapped_message.lines() {
            execute!(self.writer, PrintStyledContent("│".with(self.border_color)))?;
            for _ in 0..self.padding {
                execute!(self.writer, Print(" "))?;
            }
            execute!(self.writer, PrintStyledContent(line.with(self.message_color)))?;
            for _ in 0..self.padding + (self.width - line.len()) {
                execute!(self.writer, Print(" "))?;
            }
            execute!(self.writer, PrintStyledContent("│\n".with(self.border_color)))?;
        }

        execute!(self.writer, PrintStyledContent("└".with(self.border_color)))?;
        for _ in 0..total_width - 2 {
            execute!(self.writer, PrintStyledContent("─".with(self.border_color)))?;
        }
        execute!(self.writer, PrintStyledContent("┘\n".with(self.border_color)))?;

        self.writer.flush()?;
        Ok(())
    }
}
