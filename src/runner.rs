use crate::project::{Project, Step};
use crate::term::Term;
use std::env;
use std::io::ErrorKind;
use std::process::{Command, Stdio};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RunnerError {
    #[error("Program not found: {0}")]
    ProgramNotFound(String),
    #[error("The required program is not found: {0}")]
    RequiredNotFound(String),
    #[error("Program field for custom shell options is empty.")]
    CustomShellProgramEmpty,
    #[error("Args field for custom shell options is empty.")]
    CustomShellArgsEmpty,
    #[error("Args option for custom shell is incorrect. It should contain item with braces.")]
    CustomShellArgsIncorrect,
    #[error("Program was interrupted.")]
    Interrupted,
    #[error("Progrma has exited with a bad exit code: {0}")]
    BadExitCode(String),
    #[error("Unexpected error occured and program had exited.")]
    Unexpected,
}

pub struct Runner {
    project: Project,
}

impl Runner {
    pub fn new(project: Project) -> Self {
        Self { project }
    }

    pub fn run_project(&self) -> Result<(), RunnerError> {
        Term::message(format!("Running project '{}'.", self.project.get_name()).as_str());
        if !self.project.is_required_empty() {
            self.check_required()?;
        }
        self.project
            .get_jobs()
            .iter()
            .try_for_each(|job| self.run_job(job))
    }

    pub fn check_required(&self) -> Result<(), RunnerError> {
        for prog in self.project.get_require().iter() {
            if which::which(prog).is_err() {
                return Err(RunnerError::RequiredNotFound(prog.to_owned()));
            }
        }
        Ok(())
    }

    pub fn run_job(&self, job: (&String, &Vec<Step>)) -> Result<(), RunnerError> {
        Term::work(format!("Job '{}'.", job.0).as_str());

        job.1.iter().try_for_each(|step| self.run_step(step))
    }

    pub fn run_step(&self, step: &Step) -> Result<(), RunnerError> {
        if step.is_name_empty() && step.is_run_empty() {
            return Ok(());
        }

        let message = if !step.is_name_empty() {
            step.get_name()
        } else {
            step.get_run()
        };
        Term::work_with_margin(message.as_str());

        let mut cmd = if env::consts::OS == "windows" {
            let mut command = Command::new("cmd");
            command.arg("/C");
            command
        } else {
            let mut command = Command::new("sh");
            command.arg("-c");
            command
        };

        let with_options = self.project.get_with_options();

        if !self.project.is_with_empty() {
            if let Some(shell) = with_options.shell {
                if shell.program.is_empty() {
                    return Err(RunnerError::CustomShellProgramEmpty);
                }

                cmd = Command::new(shell.program);

                if shell.args.is_empty() {
                    return Err(RunnerError::CustomShellArgsEmpty);
                }

                if !shell.args.iter().any(|i| i == "{}") {
                    return Err(RunnerError::CustomShellArgsIncorrect);
                }

                for arg in shell.args {
                    cmd.arg(if arg == "{}" { step.get_run() } else { arg });
                }
            } else {
                cmd.arg(step.get_run());
            }

            if let Some(cwd) = with_options.cwd {
                if !cwd.is_empty() {
                    cmd.current_dir(cwd);
                }
            }
        } else {
            cmd.arg(step.get_run());
        }

        if !step.get_quite() {
            cmd.stdin(Stdio::inherit());
            cmd.stdout(Stdio::inherit());
            cmd.stderr(Stdio::inherit());
        }

        match cmd.output() {
            Ok(output) => {
                if output.status.success() {
                    Ok(())
                } else {
                    let status_code = output.status.code().unwrap_or(-1).to_string();
                    Err(RunnerError::BadExitCode(status_code))
                }
            }
            Err(error) => match error.kind() {
                ErrorKind::NotFound => Err(RunnerError::ProgramNotFound(step.get_run())),
                ErrorKind::Interrupted => Err(RunnerError::Interrupted),
                _ => Err(RunnerError::Unexpected),
            },
        }
    }
}
