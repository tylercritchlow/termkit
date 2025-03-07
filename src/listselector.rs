use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    style::{Print, Stylize},
    terminal::{size, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    cell::{Cell, RefCell},
    io::{stdout, Stdout, Write},
};

pub struct ListSelector<W: Write = Stdout> {
    pub options: Vec<String>,
    pub selected_index: Cell<usize>,
    pub top_visible_index: Cell<usize>,
    pub writer: RefCell<W>,
}

impl ListSelector<Stdout> {
    pub fn new(options: Vec<String>) -> Self {
        Self {
            options,
            selected_index: Cell::new(0),
            top_visible_index: Cell::new(0),
            writer: RefCell::new(stdout()),
        }
    }
}

impl<W: Write> ListSelector<W> {
    /// Sets a custom writer (for testing or redirection).
    pub fn with_custom_writer<W2: Write>(self, writer: W2) -> ListSelector<W2> {
        ListSelector {
            options: self.options,
            selected_index: self.selected_index,
            top_visible_index: self.top_visible_index,
            writer: RefCell::new(writer),
        }
    }

    pub fn get_selected_option(&self) -> Option<&str> {
        self.options
            .get(self.selected_index.get())
            .map(String::as_str)
    }

    pub fn render(&self) {
        let mut writer = self.writer.borrow_mut();
        let (_cols, rows) = size().unwrap();

        let num_visible_options = (rows - 1) as usize; // Leave a line for the cursor
        let start_index = self.top_visible_index.get();
        let end_index = (start_index + num_visible_options).min(self.options.len());

        for i in start_index..end_index {
            let y = (i - start_index) as u16;
            execute!(writer, cursor::MoveTo(0, y), Clear(ClearType::CurrentLine),).unwrap();
            if i == self.selected_index.get() {
                execute!(writer, Print(format!("> {}", self.options[i]).reverse()),).unwrap();
            } else {
                execute!(writer, Print(self.options[i].clone()),).unwrap();
            }
        }
        writer.flush().unwrap();
    }

    pub fn run(&self) -> Result<Option<&str>, Box<dyn std::error::Error>> {
        enable_raw_mode()?;
        {
            let mut writer = self.writer.borrow_mut();
            execute!(
                writer,
                EnterAlternateScreen,
                cursor::Hide,
                Clear(ClearType::All)
            )?;
        }

        self.render();

        loop {
            match read()? {
                Event::Key(KeyEvent {
                    code,
                    kind: KeyEventKind::Press,
                    ..
                }) => match code {
                    KeyCode::Up | KeyCode::Char('j') | KeyCode::Char('J') => {
                        if self.selected_index.get() > 0 {
                            self.selected_index.set(self.selected_index.get() - 1);
                            if self.selected_index.get() < self.top_visible_index.get() {
                                self.top_visible_index.set(self.top_visible_index.get() - 1);
                            }
                        }
                        self.render();
                    }
                    KeyCode::Down | KeyCode::Char('k') | KeyCode::Char('K') => {
                        if self.selected_index.get() < self.options.len() - 1 {
                            self.selected_index.set(self.selected_index.get() + 1);
                            let (_, rows) = size().unwrap();
                            let max_visible_index = (self.top_visible_index.get()
                                + (rows - 2) as usize)
                                .min(self.options.len() - 1);
                            if self.selected_index.get() > max_visible_index {
                                self.top_visible_index.set(self.top_visible_index.get() + 1);
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

        {
            let mut writer = self.writer.borrow_mut();
            disable_raw_mode()?;
            execute!(writer, LeaveAlternateScreen, cursor::Show)?;
        }

        Ok(self.get_selected_option())
    }
}
