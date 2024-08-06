use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear};
use crossterm::{cursor, execute, style::Print, terminal::ClearType};
use std::io::{stdout, Write};
use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use std::thread;
use std::time::Duration;

pub struct Spinner {
    frames: Vec<&'static str>,
    is_spinning: Arc<AtomicBool>, 
    position: (u16, u16),
}

impl Spinner {
    pub fn new() -> Self {
        let (x, y) = cursor::position().unwrap();
        Self {
            frames: vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            is_spinning: Arc::new(AtomicBool::new(false)),
            position: (x, y),
        }
    }

    pub fn render(&self) {
        let is_spinning = self.is_spinning.clone();
        let position = self.position;
        let frames = self.frames.clone();
        
        enable_raw_mode().unwrap();

        is_spinning.store(true, Ordering::Relaxed);

        thread::spawn(move || { 
            let mut current_frame = 0;
            while is_spinning.load(Ordering::Relaxed) {
                let frame = frames[current_frame];
                let mut stdout = stdout();
                execute!(
                    stdout,
                    cursor::MoveTo(position.0, position.1),
                    Clear(ClearType::CurrentLine),
                    Print(frame)
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

        disable_raw_mode().unwrap();
    }
}