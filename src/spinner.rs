pub use crate::spinner::LabelPosition::*;
use crossterm::cursor::Hide;
use crossterm::cursor::Show;
use crossterm::terminal::Clear;
use crossterm::{cursor, execute, style::Print, terminal::ClearType};
use std::io::{stdout, Write};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

#[derive(Clone)]
pub enum LabelPosition {
    Before,
    After,
}

pub struct Spinner {
    frames: Vec<&'static str>,
    is_spinning: Arc<AtomicBool>,
    position: (u16, u16),
    label: Option<String>,
    label_position: LabelPosition,
}

impl Spinner {
    pub fn new() -> Self {
        let (x, y) = cursor::position().unwrap();
        Self {
            frames: vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            is_spinning: Arc::new(AtomicBool::new(false)),
            position: (x, y),
            label: None,
            label_position: After,
        }
    }

    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    pub fn with_label_position(mut self, label_position: LabelPosition) -> Self {
        self.label_position = label_position;
        self
    }

    pub fn start(&self) {
        let is_spinning = self.is_spinning.clone();
        let position: (u16, u16) = self.position;
        let frames = self.frames.clone();
        let label = self.label.clone().unwrap_or("".to_string());
        let label_position = self.label_position.clone();

        is_spinning.store(true, Ordering::Relaxed);

        thread::spawn(move || {
            let mut current_frame = 0;
            while is_spinning.load(Ordering::Relaxed) {
                let frame = frames[current_frame];
                let mut stdout = stdout();
                execute!(
                    stdout,
                    Hide,
                    cursor::MoveTo(position.0, position.1),
                    Clear(ClearType::CurrentLine),
                    match label_position {
                        Before => {
                            Print(format!(" {} {}", label, frame))
                        }
                        After => {
                            Print(format!(" {} {}", frame, label))
                        }
                    }
                )
                .unwrap();
                stdout.flush().unwrap();
                current_frame = (current_frame + 1) % frames.len();
                thread::sleep(Duration::from_millis(100));
            }
        });
    }

    pub fn stop(&self) {
        execute!(
            stdout(),
            Show,
            cursor::MoveTo(self.position.0, self.position.1),
            Clear(ClearType::CurrentLine)
        )
        .unwrap();
        self.is_spinning.store(false, Ordering::Relaxed);
    }
}
