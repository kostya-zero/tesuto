use crate::term::Term;
use std::io::ErrorKind;
use std::process::{Command, Stdio};

pub struct Runner;
impl Runner {
    pub fn run_command(program: &str, args: Vec<String>) -> bool {
        let mut cmd = Command::new(program);
        cmd.args(args);
        cmd.stdin(Stdio::inherit());
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());

        match cmd.output() {
            Ok(status) => {
                if status.status.success() {
                    true
                } else if status.status.code().unwrap() != 0 {
                    Term::error("Program finished with bad exit code.");
                    return false;
                } else {
                    Term::error("Program finished with good exit code, but failed.");
                    return false;
                }
            }
            Err(error) => {
                match error.kind() {
                    ErrorKind::NotFound => Term::error("Program not found."),
                    ErrorKind::Interrupted => Term::error("Program was interrupted."),
                    ErrorKind::Other => Term::error("Program exited with unknown reason."),
                    _ => Term::error("Program exited with unknown reason."),
                };
                false
            }
        }
    }
}
