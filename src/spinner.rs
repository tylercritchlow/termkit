use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, size};
use crossterm::{cursor, execute, style::Print, terminal::ClearType, queue};
use std::io::{stdout, Write};
use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use std::thread;
use std::time::Duration;

pub struct Spinner {
    frames: Vec<&'static str>,
    is_spinning: Arc<AtomicBool>, 
    label: String,
}

impl Spinner {
    pub fn new(label: String) -> Self {
        Self {
            frames: vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            is_spinning: Arc::new(AtomicBool::new(false)),
            label,
        }
    }

    pub fn render(&self) {
        let is_spinning = self.is_spinning.clone();
        let frames = self.frames.clone();
        let label = self.label.clone();
        
        enable_raw_mode().unwrap();

        is_spinning.store(true, Ordering::Relaxed);

        thread::spawn(move || { 
            let mut current_frame = 0;
            while is_spinning.load(Ordering::Relaxed) {
                let frame = frames[current_frame];
                let mut stdout = stdout();
                let (width, height) = size().unwrap();
                execute!(
                    stdout,
                    cursor::MoveTo(0, height - 1),
                    cursor::Hide,
                    Clear(ClearType::CurrentLine),
                    Print(format!("{} {}", frame, label))
                )
                .unwrap();
                stdout.flush().unwrap();
                current_frame = (current_frame + 1) % frames.len();
                thread::sleep(Duration::from_millis(100));
            }
        });
    }

    pub fn stop(&self) {
        print!("{esc}c", esc = 27 as char);
        self.is_spinning.store(false, Ordering::Relaxed);

        execute!(stdout(), Clear(ClearType::FromCursorDown)).unwrap();

        disable_raw_mode().unwrap();
    }
}