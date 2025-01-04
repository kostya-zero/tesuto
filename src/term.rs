use std::process;

#[derive(Debug)]
pub struct Term;

impl Term {
    const RESET: &'static str = "\x1b[0m";
    const RED: &'static str = "\x1b[91m";
    const YELLOW: &'static str = "\x1b[93m";
    const GREEN: &'static str = "\x1b[92m";

    fn print_colored(icon: &str, color: &str, msg: &str) {
        println!("{color}{icon} {msg}{}", Self::RESET);
    }

    pub fn message(msg: &str) {
        println!("󰍡 {}", msg);
    }

    pub fn no_icon_message(msg: &str) {
        println!(" {}", msg);
    }

    pub fn job_name(msg: &str) {
        println!(" {}", msg);
    }

    pub fn error(msg: &str) {
        Self::print_colored("", Self::RED, msg);
    }

    pub fn fail(msg: &str) -> ! {
        Self::error(msg);
        process::exit(1);
    }

    pub fn work_with_margin(msg: &str) {
        println!("   {}", msg);
    }

    pub fn warn(msg: &str) {
        Self::print_colored("", Self::YELLOW, msg);
    }

    pub fn done(msg: &str) {
        Self::print_colored("", Self::GREEN, msg);
    }
}
