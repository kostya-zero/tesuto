use std::io::{self, Write};

pub struct Term;
impl Term {
    pub fn success(msg: &str) {
        println!("\x1b[1m\x1b[92m \x1b[0m\x1b[1m {}\x1b[0m", msg);
    }

    pub fn title(msg: &str) {
        println!("\x1b[1m\x1b[92m \x1b[0m\x1b[1m {}\x1b[0m", msg);
    }

    pub fn ask(msg: &str, default_answer: &str) -> String {
        print!("\x1b[1m\x1b[92m \x1b[0m\x1b[1m {} (default: {}): \x1b[0m", msg, default_answer);
        io::stdout().flush().expect("Failed to push content to stdout.");
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("Failed to read stdin.");
        if answer.is_empty() {
            return default_answer.to_string();
        }
        answer.trim().to_string()
    }

    pub fn list_item(name: &str, desc: &str) {
        println!("  \x1b[1m {}\x1b[0m {}", name, desc);
    }

    pub fn message(msg: &str) {
        println!(" \x1b[1m {}\x1b[0m", msg);
    }

    pub fn hint(msg: &str) {
        println!("\x1b[1m\x1b[96m 󰌵\x1b[0m\x1b[1m {}\x1b[0m", msg);
    }

    pub fn message_with_icon(msg: &str, icon: &str) {
        println!("\x1b[1m\x1b[93m {}\x1b[0m\x1b[1m {}\x1b[0m", icon, msg);
    }

    pub fn info(msg: &str) {
        println!("\x1b[1m\x1b[96m \x1b[0m\x1b[1m {}\x1b[0m", msg);
    }

    pub fn work(msg: &str) {
        println!("\x1b[1m\x1b[96m \x1b[0m\x1b[1m {}\x1b[0m", msg);
    }

    pub fn warn(msg: &str) {
        println!("\x1b[1m\x1b[93m \x1b[0m\x1b[1m {}\x1b[0m", msg);
    }

    pub fn fatal(msg: &str) {
        println!("\x1b[1m\x1b[91m \x1b[0m\x1b[1m {}\x1b[0m", msg);
    }
}
