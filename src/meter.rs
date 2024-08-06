use crossterm::{
    cursor, event, execute, queue,
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
    initial_line: u16,
}

impl Meter {
    pub fn new(label: String, max_value: f64, bar_color: Option<Color>) -> Self {
        let (_, rows) = size().unwrap();
        Self {
            label,
            value: 0.0,
            max_value,
            bar_color: bar_color.unwrap_or(Color::Green),
            initial_line: rows - 1,
        }
    }

    fn render(&self) {
        let mut stdout = stdout();
        let (cols, rows) = size().unwrap();

        // Space Management
        let mut label_width = self.label.len() + 2;
        let mut value_width = 6;
        let mut available_width = (cols as usize).saturating_sub(label_width + value_width + 3);
        while available_width < 1 {
            if label_width > 10 {
                label_width -= 1;
            } else if value_width > 4 {
                value_width -= 1;
            } else {
                break;
            }
            available_width = (cols as usize).saturating_sub(label_width + value_width + 3);
        }

        let bar_width = available_width.max(1);
        let filled_width = ((self.value / self.max_value) * bar_width as f64).round() as usize;
        let empty_width = bar_width - filled_width;

        queue!(
            stdout,
            cursor::Hide,
            cursor::MoveTo(0, self.initial_line),
            Clear(ClearType::FromCursorDown),
            Print(format!(
                "{:label_width$} |",
                self.label.chars().take(label_width - 2).collect::<String>(),
                label_width = label_width
            )),
            PrintStyledContent("#".repeat(filled_width).with(self.bar_color)),
            Print("-".repeat(empty_width)),
            Print(format!(
                " {:value_width$.2}",
                self.value,
                value_width = value_width
            )),
            cursor::Show,
        )
        .unwrap();

        stdout.flush().unwrap();
    }

    pub fn refresh(&mut self, new_value: f64, interval_ms: u64) {
        self.value = new_value.min(self.max_value);
        self.render();
        sleep(Duration::from_millis(interval_ms));
    }
    pub fn quit(&self) {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0),).unwrap();
        stdout.flush().unwrap();
    }
}
