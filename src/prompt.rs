use crossterm::event::KeyEventKind;
use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, Stylize},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{stdout, Write};

pub struct Prompt {
    prompt: String,
    input_options: Vec<String>,
    selected_index: usize,
}

impl Prompt {
    pub fn new(prompt: String, options: Vec<String>) -> Self {
        Self {
            prompt,
            input_options: options,
            selected_index: 0,
        }
    }

    pub fn get_selected_option(&self) -> Option<&str> {
        self.input_options
            .get(self.selected_index)
            .map(String::as_str)
    }

    pub fn render(&self) {
        let mut stdout = stdout();
        execute!(stdout, cursor::MoveTo(0, 0), Clear(ClearType::All)).unwrap();

        self.render_bordered_box();

        let mut x = 2;
        for (i, option) in self.input_options.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(x, 3)).unwrap();
            if i == self.selected_index {
                execute!(stdout, Print("> ".with(Color::Yellow))).unwrap();
                execute!(stdout, Print(option.clone().with(Color::Yellow).bold())).unwrap();
            } else {
                execute!(stdout, Print("  ".with(Color::White))).unwrap();
                execute!(stdout, Print(option.clone().with(Color::White))).unwrap();
            }
            x += option.len() as u16 + 4; // Add spacing between options
        }

        execute!(stdout, cursor::MoveTo(1, 5)).unwrap();
        execute!(
            stdout,
            Print("Use ←/→ to navigate, Enter to select".with(Color::DarkGrey))
        )
        .unwrap();

        stdout.flush().unwrap();
    }

    fn calculate_border_width(&self) -> u16 {
        let total_options_width: u16 = self
            .input_options
            .iter()
            .map(|option| option.len() as u16 + 4)
            .sum();
        let prompt_width = self.prompt.len() as u16 + 2;
        std::cmp::max(total_options_width, prompt_width) // Use the larger of the two
    }

    fn render_bordered_box(&self) {
        let mut stdout = stdout();
        let border_width = self.calculate_border_width();

        // Top border
        execute!(stdout, Print("╭".with(Color::Blue))).unwrap();
        for _ in 0..border_width {
            execute!(stdout, Print("─".with(Color::Blue))).unwrap();
        }
        execute!(stdout, Print("╮".with(Color::Blue))).unwrap();

        // Prompt line
        execute!(
            stdout,
            cursor::MoveTo(1, 1),
            Print(format!(" {} ", self.prompt).with(Color::Yellow))
        )
        .unwrap();

        // Middle border (below the prompt)
        execute!(stdout, cursor::MoveTo(0, 2), Print("├".with(Color::Blue))).unwrap();
        for _ in 0..border_width {
            execute!(stdout, Print("─".with(Color::Blue))).unwrap();
        }
        execute!(stdout, Print("┤".with(Color::Blue))).unwrap();

        // Bottom border (below the options)
        execute!(stdout, cursor::MoveTo(0, 4), Print("╰".with(Color::Blue))).unwrap();
        for _ in 0..border_width {
            execute!(stdout, Print("─".with(Color::Blue))).unwrap();
        }
        execute!(stdout, Print("╯".with(Color::Blue))).unwrap();
    }

    pub fn run(&mut self) -> Result<Option<&str>, Box<dyn std::error::Error>> {
        let mut stdout = stdout();
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
                    KeyCode::Char('\n') | KeyCode::Enter => {
                        execute!(stdout, LeaveAlternateScreen, cursor::Show)?;
                        return Ok(self.get_selected_option().map(|s| s));
                    }
                    KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('H') => {
                        if self.selected_index > 0 {
                            self.selected_index -= 1;
                        }
                    }
                    KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('L') => {
                        if self.selected_index < self.input_options.len() - 1 {
                            self.selected_index += 1;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
            self.render();
        }
    }
}
