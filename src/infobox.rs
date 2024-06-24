use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Write};
use textwrap::{fill, Options};

pub struct InfoBox {
    pub title: String,
    pub message: String,
    pub width: usize,
}

impl InfoBox {
    pub fn new(title: String, message: String, width: usize) -> Self {
        Self {
            title,
            message,
            width,

        }
    }

    pub fn render(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut stdout = stdout();
        let total_width = self.width + 8;
    
        execute!(stdout, cursor::MoveToColumn(0))?;
        execute!(stdout, Clear(ClearType::UntilNewLine))?;
        execute!(stdout, Print(format!("{: <width$}", self.title, width = total_width as usize)))?; 
        execute!(stdout, Print("\n"))?; 
        execute!(stdout, Print("┌"))?;
        for _ in 0..self.width {
            execute!(stdout, Print("─"))?;
        }
        execute!(stdout, Print("┐\n"))?;
    
        // Track whether it's the first line
        let mut first_line = true;
    
        let wrapped_message = fill(&self.message, self.width);
        for line in wrapped_message.lines() {
            execute!(stdout, Print("│"))?;  // Moved this line to the start of the loop
            execute!(stdout, Print(format!("{: <width$}", line, width = self.width as usize)))?;
            execute!(stdout, Print("│\n"))?;
        }
    
        execute!(stdout, Print("└"))?;
        for _ in 0..self.width {
            execute!(stdout, Print("─"))?;
        }
        execute!(stdout, Print("┘\n"))?;
        stdout.flush()?;
    
        Ok(())
    }    
}
