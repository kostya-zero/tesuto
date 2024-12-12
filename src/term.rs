use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use std::process::exit;

const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[91m";
const CYAN: &str = "\x1b[96m";
const YELLOW: &str = "\x1b[93m";
const GREEN: &str = "\x1b[92m";

pub struct Term;
impl Term {
    fn print_with_icon(icon: &str, color: &str, msg: &str) {
        println!("{BOLD}{color}{icon}{RESET}{BOLD} {msg}{RESET}");
    }

    pub fn message(msg: &str) {
        Self::print_with_icon("󰍡", "", msg);
    }

    pub fn input(message: &str, default: &str) -> String {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt(message)
            .default(default.to_string())
            .show_default(!default.is_empty())
            .interact_text()
            .unwrap()
    }

    pub fn ask(question: &str, default: bool) -> bool {
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(question)
            .default(default)
            .show_default(true)
            .interact()
            .unwrap()
    }

    pub fn no_icon_message(msg: &str) {
        println!("{BOLD}  {msg}{RESET}");
    }

    pub fn error(msg: &str) {
        Self::print_with_icon("", RED, msg);
    }

    pub fn fail(msg: &str) {
        Self::error(msg);
        exit(1);
    }

    pub fn work(msg: &str) {
        Self::print_with_icon("󰦖", CYAN, msg);
    }

    pub fn work_with_margin(msg: &str) {
        println!("{BOLD}{CYAN}   󰦖{RESET}{BOLD} {msg}{RESET}");
    }

    pub fn warn(msg: &str) {
        Self::print_with_icon("", YELLOW, msg);
    }

    pub fn done(msg: &str) {
        Self::print_with_icon("", GREEN, msg);
    }
}
