use crossterm::{
    cursor, execute,
    style::{Color, Print, PrintStyledContent, Stylize},
    terminal::{size, Clear, ClearType},
};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;

pub struct Meter {
    pub label: String,
    pub value: f64,
    pub max_value: f64,
    pub bar_color: Color,
}

impl Meter {
    pub fn new(label: String, max_value: f64, bar_color: Option<Color>) -> Self {
        Self {
            label,
            value: 0.0,
            max_value,
            bar_color: bar_color.unwrap_or(Color::Green),
        }
    }

    fn render(&self) {
        let mut stdout = stdout();
        let (cols, _rows) = size().unwrap(); // +2 for padding

        let label_width = self.label.len() + 2;
        let value_width = 6;
        let bar_width = cols as usize - label_width - value_width - 3; // -3 for spacing

        let filled_width = ((self.value / self.max_value) * bar_width as f64).round() as usize;
        let empty_width = bar_width - filled_width;

        execute!(
            stdout,
            Clear(ClearType::All),
            cursor::MoveTo(0, 0),
            Print(format!(
                "{:label_width$} |",
                self.label,
                label_width = label_width
            )),
            PrintStyledContent("#".repeat(filled_width).with(self.bar_color)),
            Print("-".repeat(empty_width)),
            Print(format!(
                " {:value_width$.2}",
                self.value,
                value_width = value_width
            ))
        )
        .unwrap();
        stdout.flush().unwrap();
    }

    pub fn refresh(&mut self, new_value: f64, interval_ms: u64) {
        self.value = new_value.min(self.max_value);
        self.render();
        sleep(Duration::from_millis(interval_ms));
    }
}
