use crossterm::{
    cursor, execute,
    style::{Color, Print, PrintStyledContent, Stylize},
    terminal::{Clear, ClearType},
};
use std::{
    cell::RefCell,
    io::{stdout, Stdout, Write},
};

pub struct Table<W: Write = Stdout> {
    pub title: String,
    pub rows: i16,
    pub columns: i16,
    pub data: RefCell<Vec<Vec<String>>>,
    pub border_color: Color,
    pub title_color: Color,
    pub data_color: Color,
    pub writer: RefCell<W>,
}

impl Table<Stdout> {
    pub fn new(title: String, rows: i16) -> Self {
        let data = RefCell::new(vec![vec!["".to_string(); 2 as usize]; rows as usize]); // data.unwrap_or_else(|| vec![vec!["".to_string(); 2 as usize]; rows as usize]);

        Table {
            title,
            rows,
            columns: 2,
            border_color: Color::Blue,
            title_color: Color::White,
            data,
            data_color: Color::White,
            writer: RefCell::new(stdout()),
        }
    }
}

impl<W: Write> Table<W> {
    /// Override the default column count
    pub fn with_column_count_override(self, columns: i16) -> Self {
        let mut data = self.data.into_inner();
        for row in &mut data {
            row.resize(columns as usize, "".to_string());
        }
        Table {
            data: RefCell::new(data),
            columns,
            ..self
        }
    }

    /// Set custom writer for the table (for testing or redirection)
    pub fn with_custom_writer<W2: Write>(self, writer: W2) -> Table<W2> {
        Table {
            title: self.title,
            rows: self.rows,
            columns: self.columns,
            border_color: self.border_color,
            title_color: self.title_color,
            data: RefCell::new(self.data.into_inner()),
            data_color: self.data_color,
            writer: RefCell::new(writer),
        }
    }

    /// Set the color for the border
    pub fn with_border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    /// Set the color for the title
    pub fn with_title_color(mut self, color: Color) -> Self {
        self.title_color = color;
        self
    }

    /// Set the color for the data cells
    pub fn with_data_color(mut self, color: Color) -> Self {
        self.data_color = color;
        self
    }

    /// Set custom data for the table at creation
    pub fn with_data(mut self, data: Vec<Vec<String>>) -> Self {
        self.data = RefCell::new(data);
        self
    }
    /// Set value for a specific cell
    pub fn set_cell(&self, row: i16, column: i16, value: String) {
        if row >= 0 && row < self.rows && column >= 0 && column < self.columns {
            let mut data = self.data.borrow_mut();
            data[row as usize][column as usize] = value;
        }
    }

    /// Calculate the maximum width of each column based on the longest string in that column
    pub fn calculate_column_widths(&self) -> Vec<usize> {
        let data = self.data.borrow();
        let mut column_widths = vec![0; self.columns as usize];

        for col in 0..self.columns {
            let mut max_len = 0;
            for row in 0..self.rows {
                let cell_value = &data[row as usize][col as usize];
                max_len = max_len.max(cell_value.len());
            }
            column_widths[col as usize] = max_len;
        }

        column_widths
    }

    pub fn render(&self) -> Result<(), Box<dyn std::error::Error>> {
        let column_widths = self.calculate_column_widths();
        let total_width: usize = column_widths.iter().sum::<usize>() + (self.columns as usize - 1);
        let mut writer = self.writer.borrow_mut();

        execute!(writer, cursor::MoveToColumn(0))?;
        execute!(writer, Clear(ClearType::UntilNewLine))?;
        execute!(
            writer,
            PrintStyledContent(
                format!("{: <width$}", self.title, width = total_width).with(self.title_color)
            )
        )?;
        execute!(writer, Print("\n"))?;

        // Print top border
        execute!(writer, PrintStyledContent("┌".with(self.border_color)))?;
        for (i, width) in column_widths.iter().enumerate() {
            execute!(
                writer,
                PrintStyledContent(
                    format!("{:─<width$}", "", width = width).with(self.border_color)
                )
            )?;
            if i < self.columns as usize - 1 {
                execute!(writer, PrintStyledContent("┬".with(self.border_color)))?;
            }
        }
        execute!(writer, PrintStyledContent("┐\n".with(self.border_color)))?;

        // Print table rows
        let data = self.data.borrow();
        for row in 0..self.rows {
            execute!(writer, PrintStyledContent("│".with(self.border_color)))?;
            for (col, width) in column_widths.iter().enumerate() {
                let cell_value = &data[row as usize][col];
                let padded_value = format!("{:<width$}", cell_value, width = width);
                execute!(
                    writer,
                    PrintStyledContent(padded_value.with(self.data_color))
                )?;
                execute!(writer, PrintStyledContent("│".with(self.border_color)))?;
            }
            execute!(writer, Print("\n"))?;

            if row < self.rows - 1 {
                execute!(writer, PrintStyledContent("├".with(self.border_color)))?;
                for (i, width) in column_widths.iter().enumerate() {
                    execute!(
                        writer,
                        PrintStyledContent(
                            format!("{:─<width$}", "", width = width).with(self.border_color)
                        )
                    )?;
                    if i < self.columns as usize - 1 {
                        execute!(writer, PrintStyledContent("┼".with(self.border_color)))?;
                    }
                }
                execute!(writer, PrintStyledContent("┤\n".with(self.border_color)))?;
            }
        }

        // Print bottom border
        execute!(writer, PrintStyledContent("└".with(self.border_color)))?;
        for (i, width) in column_widths.iter().enumerate() {
            execute!(
                writer,
                PrintStyledContent(
                    format!("{:─<width$}", "", width = width).with(self.border_color)
                )
            )?;
            if i < self.columns as usize - 1 {
                execute!(writer, PrintStyledContent("┴".with(self.border_color)))?;
            }
        }
        execute!(writer, PrintStyledContent("┘\n".with(self.border_color)))?;

        writer.flush()?;

        Ok(())
    }
}
