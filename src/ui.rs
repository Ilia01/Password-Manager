use copypasta::{ClipboardContext, ClipboardProvider};
use crossterm::{
    event::{Event, KeyCode, read},
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use passwords::{analyzer, scorer};
use std::io;

#[derive(PartialEq)]
enum WeakPasswordChoice {
    ABORT,
    CONTINUE,
}

pub fn get_user_input() -> io::Result<String> {
    println!("Write it down then press Enter!");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub fn get_master_password_user_input() -> io::Result<String> {
    println!("Enter your master password:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub fn get_clipboard_password() -> String {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.get_contents().unwrap()
}

pub fn should_save_password(password: &str) -> bool {
    if is_password_weak(password) {
        print_alert(password);
        if let Ok(choice) = read_next_char() {
            return choice == WeakPasswordChoice::CONTINUE;
        }
        false
    } else {
        true
    }
}

fn print_alert(password: &str) {
    let alert = format!(
        "{} is a weak password. Press Enter to continue anyway. Press Q to abort and try again!",
        password
    );
    execute!(
        io::stdout(),
        SetForegroundColor(Color::Red),
        SetAttribute(Attribute::Bold),
        Print(alert),
        ResetColor,
        SetAttribute(Attribute::Reset)
    )
    .unwrap();
}

fn is_password_weak(password: &str) -> bool {
    let score = scorer::score(&analyzer::analyze(password));
    score < 80.0
}

fn read_next_char() -> io::Result<WeakPasswordChoice> {
    enable_raw_mode()?;
    let result = loop {
        match read()? {
            Event::Key(event) => match event.code {
                KeyCode::Char('q') | KeyCode::Char('Q') => break Ok(WeakPasswordChoice::ABORT),
                KeyCode::Enter => break Ok(WeakPasswordChoice::CONTINUE),
                _ => {}
            },
            _ => {}
        }
    };
    disable_raw_mode()?;
    result
}
