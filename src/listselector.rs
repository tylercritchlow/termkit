use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    style::{Print, Stylize},
    terminal::{size, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{stdout, Write};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct ListSelector {
    options: Vec<String>,
    selected_index: usize,
    top_visible_index: usize,
}

impl ListSelector {
    pub fn new(options: Vec<String>) -> Self {
        Self {
            options,
            selected_index: 0,
            top_visible_index: 0,
        }
    }

    pub fn get_selected_option(&self) -> Option<&str> {
        self.options.get(self.selected_index).map(String::as_str)
    }

    pub fn render(&self) {
        let mut stdout = stdout();
        let (_cols, rows) = size().unwrap();

        let num_visible_options = (rows - 1) as usize; // Leave a line for the cursor
        let start_index = self.top_visible_index;
        let end_index = (start_index + num_visible_options).min(self.options.len());

        for i in start_index..end_index {
            let y = (i - start_index) as u16;
            execute!(stdout, cursor::MoveTo(0, y), Clear(ClearType::CurrentLine),).unwrap();
            if i == self.selected_index {
                execute!(stdout, Print(format!("> {}", self.options[i]).reverse()),).unwrap();
            } else {
                execute!(stdout, Print(self.options[i].clone()),).unwrap();
            }
        }
        stdout.flush().unwrap();
    }

    pub fn run(&mut self) -> Result<Option<&str>, Box<dyn std::error::Error>> {
        let mut stdout = stdout();
        enable_raw_mode()?;
        execute!(
            stdout,
            EnterAlternateScreen,
            cursor::Hide,
            Clear(ClearType::All)
        )?;
        self.render();

        loop {
            match read()? {
                Event::Key(KeyEvent {
                    code,
                    kind: KeyEventKind::Press,
                    ..
                }) => match code {
                    KeyCode::Up | KeyCode::Char('j') | KeyCode::Char('J') => {
                        if self.selected_index > 0 {
                            self.selected_index -= 1;
                            if self.selected_index < self.top_visible_index {
                                self.top_visible_index -= 1;
                            }
                        }
                        self.render();
                    }
                    KeyCode::Down | KeyCode::Char('k') | KeyCode::Char('K') => {
                        if self.selected_index < self.options.len() - 1 {
                            self.selected_index += 1;
                            let (_, rows) = size().unwrap();
                            let max_visible_index = (self.top_visible_index + (rows - 2) as usize)
                                .min(self.options.len() - 1);
                            if self.selected_index > max_visible_index {
                                self.top_visible_index += 1;
                            }
                        }
                        self.render();
                    }
                    KeyCode::Enter => break,
                    _ => {}
                },
                _ => {}
            }
        }

        disable_raw_mode()?;
        execute!(stdout, LeaveAlternateScreen, cursor::Show)?;
        Ok(self.get_selected_option())
    }
}
