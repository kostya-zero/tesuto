use crate::platform::Platform;
use crate::project::{Project, Step};
use crate::term::Term;
use std::io::ErrorKind;
use std::process::{Command, Stdio};
use std::{env, fmt};

pub enum RunnerError {
    ProgramNotFound(String),
    RequiredNotFound(String),
    Interrupted,
    BadExitCode(String),
    Unexpected,
}

impl fmt::Display for RunnerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RunnerError::ProgramNotFound(prog) => write!(f, "Program not found: {prog}"),
            RunnerError::RequiredNotFound(prog) => {
                write!(f, "The required program is not found: {prog}")
            }
            RunnerError::Interrupted => write!(f, "Program was interrupted."),
            RunnerError::BadExitCode(code) => {
                write!(f, "Program has exited with a bad exit code: {code}")
            }
            RunnerError::Unexpected => {
                write!(f, "Unexpected error occured and program had exited.")
            }
        }
    }
}

pub struct Runner {
    project: Project,
}

impl Runner {
    pub fn new(new_project: Project) -> Self {
        Self {
            project: new_project,
        }
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
            if !Platform::is_program_installed(prog.as_str()) {
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

        cmd.arg(step.get_run());

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
