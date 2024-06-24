use crossterm::terminal::Clear;
use crossterm::{cursor, execute, style::Print, terminal::ClearType};
use std::io::{stdout, Write};

pub struct Spinner {
    frames: Vec<&'static str>,
    current_frame: usize,
    is_spinning: bool,
    position: (u16, u16), // Store initial cursor position
}

impl Spinner {
    pub fn new() -> Self {
        let (x, y) = cursor::position().unwrap(); // Get initial position
        Self {
            frames: vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            current_frame: 0,
            is_spinning: false,
            position: (x, y), // Store initial position
        }
    }

    pub fn start(&mut self) {
        self.is_spinning = true;
        self.render().unwrap();
    }

    pub fn stop(&mut self) {
        self.is_spinning = false;
        execute!(stdout(), Clear(ClearType::CurrentLine)).unwrap();
    }

    fn render(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut stdout = stdout();
        execute!(stdout, cursor::MoveTo(self.position.0, self.position.1))?; // Move back to initial position
        execute!(
            stdout,
            Clear(ClearType::UntilNewLine),
            Print(self.frames[self.current_frame])
        )?;
        stdout.flush()?;

        self.current_frame = (self.current_frame + 1) % self.frames.len();
        if self.is_spinning {
            std::thread::sleep(std::time::Duration::from_millis(100));
            self.render()?;
        }
        Ok(())
    }
}
