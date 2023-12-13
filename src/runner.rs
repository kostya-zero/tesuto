use crate::term::Term;
use std::io::ErrorKind;
use std::process::{Command, Stdio};
use crate::project::Action;

pub struct Runner;
impl Runner {
    pub fn run_action(action: Action) -> bool {
        if !action.is_name_empty() {
            Term::message(action.get_name().as_str());
        } else if !action.is_program_empty() {
            Term::message(format!("Running `{}`", action.get_program()).as_str());
        } else {
            return true;
        }

        if !action.is_program_empty() {
            let command = action.get_program();
            let mut args: Vec<String> = Vec::new();
            if !action.is_args_empty() {
                args = action.get_args();
            }
            if !Runner::run_command(command.as_str(), args) {
                Term::error("Action failed to run.");
                return false;
            }
        }
        true
    }
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
                    ErrorKind::NotFound => Term::error(format!("Cannot found required program: {}.", program).as_str()),
                    ErrorKind::Interrupted => Term::error("Program was interrupted while running."),
                    ErrorKind::Other => Term::error("Program exited with unknown reason."),
                    _ => Term::error("Program exited with unknown reason."),
                };
                false
            }
        }
    }
}
