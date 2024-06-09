use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use std::process::exit;

pub struct Term;
impl Term {
    pub fn message(msg: &str) {
        println!("\x1b[1m 󰍡 {msg}\x1b[0m");
    }

    pub fn input(message: &str, default: &str) -> String {
        let response = Input::with_theme(&ColorfulTheme::default())
            .with_prompt(message)
            .default(default.to_string())
            .show_default(!default.is_empty())
            .interact_text()
            .unwrap();
        response
    }

    pub fn ask(question: &str, default: bool) -> bool {
        let response = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(question)
            .default(default)
            .show_default(true)
            .interact()
            .unwrap();
        response
    }

    pub fn no_icon_message(msg: &str) {
        println!("\x1b[1m   {msg}\x1b[0m");
    }

    pub fn error(msg: &str) {
        println!("\x1b[1m\x1b[91m \x1b[0m\x1b[1m {msg}\x1b[0m");
    }

    pub fn fail(msg: &str) {
        println!("\x1b[1m\x1b[91m \x1b[0m\x1b[1m {msg}\x1b[0m");
        exit(1)
    }

    pub fn work(msg: &str) {
        println!("\x1b[1m\x1b[96m 󰦖\x1b[0m\x1b[1m {msg}\x1b[0m");
    }

    pub fn work_with_margin(msg: &str) {
        println!("\x1b[1m\x1b[96m   󰦖\x1b[0m\x1b[1m {msg}\x1b[0m");
    }

    pub fn warn(msg: &str) {
        println!("\x1b[1m\x1b[93m \x1b[0m\x1b[1m {msg}\x1b[0m");
    }

    pub fn done(msg: &str) {
        println!("\x1b[1m\x1b[92m \x2b[0m\x1b[1m {msg}\x1b[0m");
    }
}
