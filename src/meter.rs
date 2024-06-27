use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent},
    execute,
    style::{Print, Stylize},
    terminal::{
        disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use std::io::{stdout, Write};

pub struct Meter {
    pub value: f64,
    max_value: f64,
    label: String,
}

impl Meter {
    pub fn new(value: f64, max_value: f64, label: String) -> Self {
        Self {
            value,
            max_value,
            label,
        }
    }
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen, cursor::Hide, Clear(ClearType::All))?;
        self.render();

        loop {
            match read()? {
                Event::Key(KeyEvent { code, .. }) => match code {
                    KeyCode::Char('q') => break,
                    _ => {}
                },
                _ => {}
            }
        }

        execute!(stdout, LeaveAlternateScreen, cursor::Show)?;
        disable_raw_mode()?;
        Ok(())
    }
    pub fn render(&self) {
        let mut stdout = stdout();
        let (cols, _rows) = size().unwrap();

        // Smarter layout
        let label_width = self.label.len() + 2; // Label + spacing
        let value_width = 6; // Enough for most values
        let bar_width = cols as usize - label_width - value_width - 3; // -3 for spacing

        let filled_width = ((self.value / self.max_value) * bar_width as f64).round() as usize;
        let empty_width = bar_width - filled_width;

        execute!(
            stdout,
            cursor::MoveTo(0, 0),
            Clear(ClearType::CurrentLine),
            Print(format!(
                "{:label_width$} |",
                self.label,
                label_width = label_width
            )), // Pad label
            Print("#".repeat(filled_width).green()),
            Print("-".repeat(empty_width)),
            Print(format!(
                " {:value_width$.2}",
                self.value,
                value_width = value_width
            )) // Display value
        )
        .unwrap();
        stdout.flush().unwrap();
    }

    pub fn refresh(&self) {
        let mut stdout = stdout();
        execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
        self.render();
    }
}
