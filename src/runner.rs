use crate::project::{Project, Step};
use crate::term::Term;
use std::env;
use std::io::ErrorKind;
use std::process::{Command, Stdio};

pub enum RunnerError {
    ProgramNotFound(String),
    Interrupted,
    BadExitCode(String),
    DoneButFailed,
    Unknown,
}

pub struct Runner {
    project: Project
}

impl Runner {
    pub fn new(new_project: Project) -> Self {
        Self { project: new_project }
    }

    pub fn run_project(&self) -> Result<(), RunnerError> {
        Term::message(format!("Running project '{}'.", self.project.get_name()).as_str());
        self.project.get_jobs().iter().try_for_each(|job| self.run_job(job))
    }

    pub fn run_job(&self, job: (&String, &Vec<Step>)) -> Result<(), RunnerError> {
        Term::work(format!("Job '{}'.", job.0).as_str());

        job.1.iter().try_for_each(|step| self.run_step(step))
    }

    pub fn run_step(&self, step: &Step) -> Result<(), RunnerError> {
        if !step.is_name_empty() || !step.is_run_empty() {
            let message = if !step.is_name_empty() {
                step.get_name()
            } else {
                step.get_run()
            };
            Term::work_margin(message.as_str());
        } else {
            return Ok(());
        }

        #[allow(unused_assignments)]
        let mut cmd = Command::new("");
        match env::consts::OS {
            "windows" => {
                cmd = Command::new("cmd");
                cmd.arg("/C");
            },
            _ => {
                cmd = Command::new("sh");
                cmd.arg("-c");
            }
        }

        cmd.arg(step.get_run());
        if !step.get_quite() {
            cmd.stdin(Stdio::inherit());
            cmd.stdout(Stdio::inherit());
            cmd.stderr(Stdio::inherit());
        }

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
            },
            Err(error) => match error.kind() {
                ErrorKind::NotFound => Err(RunnerError::ProgramNotFound(step.get_run())),
                ErrorKind::Interrupted => Err(RunnerError::Interrupted),
                ErrorKind::Other => Err(RunnerError::Unknown),
                _ => Err(RunnerError::Unknown),
            },
        }
        
    }

}