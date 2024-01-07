use crate::term::Term;
use std::io::ErrorKind;
use std::process::{Command, Stdio};
use crate::project::Action;

pub enum RunnerError {
    ProgramNotFound(String),
    Interrupted,
    BadExitCode(String),
    DoneButFailed,
    Unknown,
}

pub struct Runner;
impl Runner {
    pub fn run_action(action: Action) -> Result<(), RunnerError> {
        if !action.is_name_empty() {
            Term::message(action.get_name().as_str());
        } else if !action.is_program_empty() {
            Term::message(format!("Running `{}`", action.get_program()).as_str());
        } else {
            return Ok(());
        }

        if !action.is_program_empty() {
            let command = action.get_program();
            let mut args: Vec<String> = Vec::new();
            if !action.is_args_empty() {
                args = action.get_args();
            }
            return Runner::run_command(command.as_str(), args);
        } else {
            Ok(())
        }
    }

    pub fn run_command(program: &str, args: Vec<String>) -> Result<(), RunnerError> {
        let mut cmd = Command::new(program);
        cmd.args(args);
        cmd.stdin(Stdio::inherit());
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());

        match cmd.output() {
            Ok(status) => {
                if status.status.success() {
                    Ok(())
                } else if status.status.code().unwrap() != 0 {
                    let status_code = status.status.code().unwrap().to_string();
                    Err(RunnerError::BadExitCode(status_code))
                } else {
                    Err(RunnerError::DoneButFailed)
                }
            }
            Err(error) => {
                match error.kind() {
                    ErrorKind::NotFound => Err(RunnerError::ProgramNotFound(String::from(program))),
                    ErrorKind::Interrupted => Err(RunnerError::Interrupted),
                    ErrorKind::Other => Err(RunnerError::Unknown),
                    _ => Err(RunnerError::Unknown),
                }
            }
        }
    }
}
