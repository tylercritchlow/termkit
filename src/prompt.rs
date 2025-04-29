use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    style::{Color, Print, PrintStyledContent, Stylize},
    terminal::{self, Clear, ClearType},
};
use std::{
    cell::{Cell, RefCell},
    io::{stdout, Stdout, Write},
};

use crate::RawModeGuard;

pub struct Prompt<W: Write = Stdout> {
    pub message: String,
    pub options: Vec<String>,
    pub selected_index: Cell<usize>,
    pub writer: RefCell<W>,
    pub message_color: Color,
    pub arrow_color: Color,
    pub selected_option_color: Color,
    pub unselected_option_color: Color,
    pub diamond_color: Color,
    pub question_mark_color: Color,
}

impl Prompt<Stdout> {
    pub fn new(message: String, options: Vec<String>) -> Self {
        Self {
            message,
            options,
            selected_index: Cell::new(0),
            writer: RefCell::new(stdout()),
            message_color: Color::Grey,
            arrow_color: Color::DarkGreen,
            selected_option_color: Color::Yellow,
            unselected_option_color: Color::White,
            diamond_color: Color::Green,
            question_mark_color: Color::Green,
        }
    }
}

impl<W: Write> Prompt<W> {
    pub fn with_custom_writer<W2: Write>(self, writer: W2) -> Prompt<W2> {
        Prompt {
            message: self.message,
            options: self.options,
            selected_index: self.selected_index,
            writer: RefCell::new(writer),
            message_color: self.message_color,
            arrow_color: self.arrow_color,
            selected_option_color: self.selected_option_color,
            unselected_option_color: self.unselected_option_color,
            diamond_color: self.diamond_color,
            question_mark_color: self.question_mark_color,
        }
    }

    pub fn with_colors(
        mut self,
        message_color: Color,
        arrow_color: Color,
        selected_option_color: Color,
        unselected_option_color: Color,
        diamond_color: Color,
        question_mark_color: Color,
    ) -> Self {
        self.message_color = message_color;
        self.arrow_color = arrow_color;
        self.selected_option_color = selected_option_color;
        self.unselected_option_color = unselected_option_color;
        self.diamond_color = diamond_color;
        self.question_mark_color = question_mark_color;
        self
    }

    pub fn get_selected_option(&self) -> Option<&str> {
        self.options
            .get(self.selected_index.get())
            .map(String::as_str)
    }

    fn setup(&self) -> Result<(), Box<dyn std::error::Error>> {
        let (_, rows) = terminal::size()?;
        let lines_needed = self.options.len() + 2;
        let cursor_pos = cursor::position()?.1;

        if cursor_pos + lines_needed as u16 > rows {
            let mut writer = self.writer.borrow_mut();
            for _ in 0..(lines_needed - (rows - cursor_pos as u16) as usize) {
                execute!(writer, Print("\n"))?;
            }
        }
        Ok(())
    }

    pub fn render(&self) {
        let mut writer = self.writer.borrow_mut();
        let lines_needed = self.options.len() + 2;

        execute!(
            writer,
            cursor::MoveUp(lines_needed as u16 - 1),
            Clear(ClearType::FromCursorDown)
        )
        .unwrap();

        execute!(
            writer,
            PrintStyledContent("?".with(self.question_mark_color)),
            Print(" "),
            PrintStyledContent(self.message.clone().with(self.message_color)),
            cursor::MoveToNextLine(1),
        )
        .unwrap();

        for (i, option) in self.options.iter().enumerate() {
            if i == self.selected_index.get() {
                execute!(
                    writer,
                    PrintStyledContent("♦".with(self.diamond_color)),
                    Print(" "),
                    PrintStyledContent(option.clone().with(self.selected_option_color)),
                    cursor::MoveToNextLine(1),
                )
                .unwrap();
            } else {
                execute!(
                    writer,
                    PrintStyledContent(format!("  {}", option).with(self.unselected_option_color)),
                    cursor::MoveToNextLine(1),
                )
                .unwrap();
            }
        }

        writer.flush().unwrap();
    }

    pub fn run(&self) -> Result<Option<&str>, Box<dyn std::error::Error>> {
        let _raw_mode_guard = RawModeGuard::new()?;

        {
            let mut writer = self.writer.borrow_mut();
            execute!(writer, cursor::Hide)?;
        }

        self.setup()?;
        self.render();

        loop {
            match read()? {
                Event::Key(KeyEvent {
                    code,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    let prev_index = self.selected_index.get();
                    match code {
                        KeyCode::Up => {
                            if prev_index > 0 {
                                self.selected_index.set(prev_index - 1);
                            }
                        }
                        KeyCode::Down => {
                            if prev_index < self.options.len() - 1 {
                                self.selected_index.set(prev_index + 1);
                            }
                        }
                        KeyCode::Enter => break,
                        _ => {}
                    }
                    if self.selected_index.get() != prev_index {
                        self.render();
                    }
                }
                _ => {}
            }
        }

        {
            let mut writer = self.writer.borrow_mut();
            let lines_needed = self.options.len() + 2;
            let selected_option = self.get_selected_option().unwrap_or("");

            execute!(
                writer,
                cursor::MoveUp(lines_needed as u16 - 1),
                Clear(ClearType::FromCursorDown),
                PrintStyledContent("?".with(self.question_mark_color)),
                Print(" "),
                PrintStyledContent(self.message.clone().with(self.message_color)),
                Print(" "),
                PrintStyledContent("❯".with(self.arrow_color)),
                Print(" "),
                PrintStyledContent(selected_option.with(self.selected_option_color)),
                cursor::MoveToNextLine(1),
                cursor::Show
            )?;
        }

        Ok(self.get_selected_option())
    }
}