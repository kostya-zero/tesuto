use std::process::exit;

pub struct Term;
impl Term {
    pub fn message(msg: &str) {
        println!("\x1b[1m 󰍡 {}\x1b[0m", msg);
    }

    pub fn no_icon_message(msg: &str) {
        println!("\x1b[1m   {}\x1b[0m", msg);
    }

    pub fn error(msg: &str) {
        println!("\x1b[1m\x1b[91m \x1b[0m\x1b[1m {}\x1b[0m", msg);
    }

    pub fn fail(msg: &str) {
        println!("\x1b[1m\x1b[91m \x1b[0m\x1b[1m {}\x1b[0m", msg);
        exit(1)
    }

    pub fn work(msg: &str) {
        println!("\x1b[1m\x1b[91m 󰦖\x1b[0m\x1b[1m {}\x1b[0m", msg);
    }

    pub fn work_margin(msg: &str) {
        println!("\x1b[1m\x1b[91m   󰦖\x1b[0m\x1b[1m {}\x1b[0m", msg);
    }

    pub fn warn(msg: &str) {
        println!("\x1b[1m\x1b[93m \x1b[0m\x1b[1m {}\x1b[0m", msg);
    }

    pub fn done(msg: &str) {
        println!("\x1b[1m\x1b[92m \x1b[0m\x1b[1m {}\x1b[0m", msg);
    }
}
