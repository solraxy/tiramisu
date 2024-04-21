use std::{fs::read_to_string, io::stdout};

use crossterm::{
    event::{read, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{Clear, ClearType},
};
struct App {
    file_content: String,
    user_input: String,
}

impl App {
    fn new(file_name: &str) -> Result<Self, std::io::Error> {
        let file_content = read_to_string(file_name)?;
        Ok(Self {
            file_content,
            user_input: String::new(),
        })
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut app = App::new("demo.txt")?;

    loop {
        println!("{:?}", app.file_content);
        for (letter_original, letter_input) in app.user_input.chars().zip(app.file_content.chars())
        {
            if letter_original == letter_input {
                print!("{letter_input}")
            } else {
                print!("*");
            }
        }
        println!("_");
        if let Event::Key(key_event) = read()? {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Backspace => {
                        app.user_input.pop();
                    }
                    KeyCode::Esc => break,
                    KeyCode::Char(c) => {
                        app.user_input.push(c);
                    }
                    KeyCode::Enter => {
                        let total_chars = app.user_input.chars().count();
                        let total_right = app
                            .user_input
                            .chars()
                            .zip(app.file_content.chars())
                            .filter(|(a, b)| a == b)
                            .count();
                        println!("{total_right}/{total_chars}");
                        return Ok(());
                    }
                    _ => {}
                }
            }
            execute!(stdout(), Clear(ClearType::All))?;
        }
    }
    Ok(())
}
