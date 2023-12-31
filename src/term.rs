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
    pub fn warn(msg: &str) {
        println!("\x1b[1m\x1b[93m \x1b[0m\x1b[1m {}\x1b[0m", msg);
    }
    pub fn done(msg: &str) {
        println!("\x1b[1m\x1b[92m \x1b[0m\x1b[1m {}\x1b[0m", msg);
    }
    pub fn traceback(job_name: &str, action_num: &str, reason: &str) {
        println!("\x1b[1m\x1b[91m \x1b[0m\x1b[1m Tesuto has catched a fail in job {job_name} action {action_num}.\x1b[0m");
        println!("\x1b[1m   Reason: {reason}\x1b[0m");
    }
}
