use crossterm::{
    cursor, execute,
    style::{Color, Print, PrintStyledContent, Stylize},
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Write};

pub struct Table {
    title: String,
    rows: i16,
    columns: i16,
    border_color: Color,
    title_color: Color,
}

impl Table {
    pub fn new(title: String, rows: i16, columns: i16, border_color: Option<Color>, title_color: Option<Color>) -> Self {        
        Table {
            title,
            rows,
            columns,
            border_color: border_color.unwrap_or(Color::Blue),
            title_color: title_color.unwrap_or(Color::White),
        }
    }
    
    pub fn render(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut stdout = stdout();
        let total_width = (self.columns as usize) * 4 - 1;
        
        execute!(stdout, cursor::MoveToColumn(0))?;
        execute!(stdout, Clear(ClearType::UntilNewLine))?;
        execute!(
            stdout,
            PrintStyledContent(format!("{: <width$}", self.title, width = total_width).with(self.title_color))
        )?;
        execute!(stdout, Print("\n"))?;
        
        execute!(stdout, PrintStyledContent("┌".with(self.border_color)))?;
        for i in 0..self.columns {
            execute!(stdout, PrintStyledContent("───".with(self.border_color)))?;
            if i < self.columns - 1 {
                execute!(stdout, PrintStyledContent("┬".with(self.border_color)))?;
            }
        }
        execute!(stdout, PrintStyledContent("┐\n".with(self.border_color)))?;

        for row in 0..self.rows {
            execute!(stdout, PrintStyledContent("│".with(self.border_color)))?;
            for _ in 0..self.columns {
                execute!(stdout, PrintStyledContent("   │".with(self.border_color)))?;
            }
            execute!(stdout, Print("\n"))?;
            
            if row < self.rows - 1 {
                execute!(stdout, PrintStyledContent("├".with(self.border_color)))?;
                for i in 0..self.columns {
                    execute!(stdout, PrintStyledContent("───".with(self.border_color)))?;
                    if i < self.columns - 1 {
                        execute!(stdout, PrintStyledContent("┼".with(self.border_color)))?;
                    }
                }
                execute!(stdout, PrintStyledContent("┤\n".with(self.border_color)))?;
            }
        }

        execute!(stdout, PrintStyledContent("└".with(self.border_color)))?;
        for i in 0..self.columns {
            execute!(stdout, PrintStyledContent("───".with(self.border_color)))?;
            if i < self.columns - 1 {
                execute!(stdout, PrintStyledContent("┴".with(self.border_color)))?;
            }
        }
        execute!(stdout, PrintStyledContent("┘\n".with(self.border_color)))?;

        stdout.flush()?;
        
        Ok(())
    }
}
