use crossterm::{
    cursor, execute,
    style::{Color, Print, PrintStyledContent, Stylize},
    terminal::{Clear, ClearType},
};
use std::cell::RefCell;
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
    pub writer: RefCell<W>, // Use RefCell for interior mutability
}

impl InfoBox<Stdout> {
    /// Creates a new `InfoBox` with default colors.
    pub fn new(title: impl Into<String>, message: impl Into<String>, width: usize) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            width,
            padding: 2, // Default padding
            title_color: Color::White,
            border_color: Color::Blue,
            message_color: Color::Reset,
            writer: RefCell::new(stdout()), // Wrap stdout in RefCell
        }
    }
}

impl<W: Write> InfoBox<W> {
    /// Sets custom padding.
    pub fn with_padding(mut self, padding: usize) -> Self {
        self.padding = padding;
        self
    }

    /// Sets a custom writer (for testing or redirection).
    pub fn with_custom_writer<W2: Write>(self, writer: W2) -> InfoBox<W2> {
        InfoBox {
            title: self.title,
            message: self.message,
            width: self.width,
            padding: self.padding,
            title_color: self.title_color,
            border_color: self.border_color,
            message_color: self.message_color,
            writer: RefCell::new(writer), // Wrap the new writer in RefCell
        }
    }

    /// Sets a custom title color.
    pub fn with_title_color(mut self, color: Color) -> Self {
        self.title_color = color;
        self
    }

    /// Sets a custom border color.
    pub fn with_border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    /// Sets a custom message color.
    pub fn with_message_color(mut self, color: Color) -> Self {
        self.message_color = color;
        self
    }

    /// Renders the InfoBox.
    pub fn render(&self) -> Result<(), Box<dyn std::error::Error>> {
        let total_width = self.width + 2 * self.padding + 2;
        let mut writer = self.writer.borrow_mut(); // Borrow the writer mutably

        execute!(writer, cursor::MoveToColumn(0))?;
        execute!(writer, Clear(ClearType::UntilNewLine))?;

        execute!(
            writer,
            PrintStyledContent(
                format!("{: <width$}", self.title, width = total_width).with(self.title_color)
            )
        )?;
        execute!(writer, Print("\n"))?;

        execute!(writer, PrintStyledContent("┌".with(self.border_color)))?;
        for _ in 0..total_width - 2 {
            execute!(writer, PrintStyledContent("─".with(self.border_color)))?;
        }
        execute!(writer, PrintStyledContent("┐\n".with(self.border_color)))?;

        let wrapped_message = fill(&self.message, self.width);
        for line in wrapped_message.lines() {
            execute!(writer, PrintStyledContent("│".with(self.border_color)))?;
            for _ in 0..self.padding {
                execute!(writer, Print(" "))?;
            }
            execute!(writer, PrintStyledContent(line.with(self.message_color)))?;
            for _ in 0..self.padding + (self.width - line.len()) {
                execute!(writer, Print(" "))?;
            }
            execute!(writer, PrintStyledContent("│\n".with(self.border_color)))?;
        }

        execute!(writer, PrintStyledContent("└".with(self.border_color)))?;
        for _ in 0..total_width - 2 {
            execute!(writer, PrintStyledContent("─".with(self.border_color)))?;
        }
        execute!(writer, PrintStyledContent("┘\n".with(self.border_color)))?;

        writer.flush()?;
        Ok(())
    }
}
