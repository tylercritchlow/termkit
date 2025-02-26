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
    data: Vec<Vec<String>>,
    border_color: Color,
    title_color: Color,
    data_color: Color,
}

impl Table {
    pub fn new(
        title: String, 
        rows: i16, 
        border_color: Option<Color>, 
        title_color: Option<Color>,
        data: Option<Vec<Vec<String>>>,
    ) -> Self {
        let data = data.unwrap_or_else(|| vec![vec!["".to_string(); 2 as usize]; rows as usize]);
        
        Table {
            title,
            rows,
            columns: 2,
            border_color: border_color.unwrap_or(Color::Blue),
            title_color: title_color.unwrap_or(Color::White),
            data,
            data_color: Color::White, // default data color
        }
    }

    /// Override the default column count
    /// This is not recommended, but can be used with cautious amounts of columns because of terminal width restrictions.
    pub fn with_column_count_override(mut self, columns: i16) -> Self {
        self.columns = columns;
        for row in &mut self.data {
            row.resize(columns as usize, "".to_string());
        }
        self
    }

    /// Set value for a specific cell
    pub fn set_cell(&mut self, row: i16, column: i16, value: String) {
        if row >= 0 && row < self.rows && column >= 0 && column < self.columns {
            self.data[row as usize][column as usize] = value;
        }
    }

    /// Calculate the maximum width of each column based on the longest string in that column
    fn calculate_column_widths(&self) -> Vec<usize> {
        let mut column_widths = vec![0; self.columns as usize];

        for col in 0..self.columns {
            let mut max_len = 0;
            for row in 0..self.rows {
                let cell_value = &self.data[row as usize][col as usize];
                max_len = max_len.max(cell_value.len());
            }
            column_widths[col as usize] = max_len;
        }

        column_widths
    }

    pub fn render(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut stdout = stdout();
        let column_widths = self.calculate_column_widths(); // Get dynamic column widths
        let total_width: usize = column_widths.iter().sum::<usize>() + (self.columns as usize - 1); // Space between columns

        execute!(stdout, cursor::MoveToColumn(0))?;
        execute!(stdout, Clear(ClearType::UntilNewLine))?;
        execute!(
            stdout,
            PrintStyledContent(format!("{: <width$}", self.title, width = total_width).with(self.title_color))
        )?;
        execute!(stdout, Print("\n"))?;

        // Print top border
        execute!(stdout, PrintStyledContent("┌".with(self.border_color)))?;
        for (i, width) in column_widths.iter().enumerate() {
            execute!(stdout, PrintStyledContent(format!("{:─<width$}", "", width = width).with(self.border_color)))?;
            if i < self.columns as usize - 1 {
                execute!(stdout, PrintStyledContent("┬".with(self.border_color)))?;
            }
        }
        execute!(stdout, PrintStyledContent("┐\n".with(self.border_color)))?;

        // Print table rows
        for row in 0..self.rows {
            execute!(stdout, PrintStyledContent("│".with(self.border_color)))?;
            for (col, width) in column_widths.iter().enumerate() {
                let cell_value = &self.data[row as usize][col];
                let padded_value = format!("{:<width$}", cell_value, width = width);
                execute!(stdout, PrintStyledContent(padded_value.with(self.data_color)))?;
                execute!(stdout, PrintStyledContent("│".with(self.border_color)))?;
            }
            execute!(stdout, Print("\n"))?;

            if row < self.rows - 1 {
                execute!(stdout, PrintStyledContent("├".with(self.border_color)))?;
                for (i, width) in column_widths.iter().enumerate() {
                    execute!(stdout, PrintStyledContent(format!("{:─<width$}", "", width = width).with(self.border_color)))?;
                    if i < self.columns as usize - 1 {
                        execute!(stdout, PrintStyledContent("┼".with(self.border_color)))?;
                    }
                }
                execute!(stdout, PrintStyledContent("┤\n".with(self.border_color)))?;
            }
        }

        // Print bottom border
        execute!(stdout, PrintStyledContent("└".with(self.border_color)))?;
        for (i, width) in column_widths.iter().enumerate() {
            execute!(stdout, PrintStyledContent(format!("{:─<width$}", "", width = width).with(self.border_color)))?;
            if i < self.columns as usize - 1 {
                execute!(stdout, PrintStyledContent("┴".with(self.border_color)))?;
            }
        }
        execute!(stdout, PrintStyledContent("┘\n".with(self.border_color)))?;

        stdout.flush()?;

        Ok(())
    }
}
