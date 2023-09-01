use std::io::ErrorKind;
use std::{
    process::{Command, Stdio},
    vec,
};
use tesuto_project::{Stage, Vars};

use crate::term::Term;

pub enum RunError {
    BadExitCode,
    Interrupted,
    ExecuteFailed,
    UnknownError,
}

pub struct Runner;
impl Runner {
    pub fn run_stage(stage: Stage) -> bool {
        if !stage.get_before_scripts().is_empty() {
            Term::message("Running before scripts...");
            for i in stage.get_before_scripts() {
                match Self::run_command(&i, stage.get_quite(), stage.get_variables()) {
                    Ok(_) => Term::done("Success."),
                    Err(err) => match err {
                        RunError::BadExitCode => {
                            Term::error("Script failed because command exited with bad exit code.");
                            return false;
                        }
                        RunError::Interrupted => {
                            Term::error("Script failed because it was interrupted.");
                            return false;
                        }
                        RunError::ExecuteFailed => {
                            Term::error("Script failed because cant run executable file.");
                            return false;
                        }
                        RunError::UnknownError => {
                            Term::error("Scritp failed with unknown reason.");
                            return false;
                        }
                    },
                }
            }
            Term::done("All before scripts executed.");
        }

        if stage.get_scripts().is_empty() {
            Term::message("Scripts not added, nothing to run.");
            true
        } else {
            Term::message("Running scripts...");
            for i in stage.get_scripts() {
                match Self::run_command(&i, stage.get_quite(), stage.get_variables()) {
                    Ok(_) => Term::done("Success."),
                    Err(err) => match err {
                        RunError::BadExitCode => {
                            Term::error("Script failed because command exited with bad exit code.");
                            return false;
                        }
                        RunError::Interrupted => {
                            Term::error("Script failed because it was interrupted.");
                            return false;
                        }
                        RunError::ExecuteFailed => {
                            Term::error("Script failed because cant run executable file.");
                            return false;
                        }
                        RunError::UnknownError => {
                            Term::error("Script failed with unknown reason.");
                            return false;
                        }
                    },
                }
            }
            Term::done("All scripts executed.");
            true
        }
    }

    fn run_command(command: &str, quite: bool, variables: Vars) -> Result<(), RunError> {
        let mut cmd = Command::new("sh");
        cmd.args(vec!["-c", command]);
        if !quite {
            cmd.stdin(Stdio::inherit());
            cmd.stdout(Stdio::inherit());
            cmd.stderr(Stdio::inherit());
        }
        for (k, v) in variables.iter() {
            cmd.env(k, v);
        }

        match cmd.output() {
            Ok(status) => {
                if status.status.success() {
                    Ok(())
                } else {
                    match status.status.code().unwrap() != 0 {
                        true => Err(RunError::BadExitCode),
                        false => Err(RunError::UnknownError),
                    }
                }
            }
            Err(error) => match error.kind() {
                ErrorKind::NotFound => Err(RunError::ExecuteFailed),
                ErrorKind::Interrupted => Err(RunError::Interrupted),
                ErrorKind::Other => Err(RunError::UnknownError),
                _ => Err(RunError::UnknownError),
            },
        }
    }
}
