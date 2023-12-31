use std::{path::Path, process::exit};

use args::args;
use clap::ArgMatches;
use manager::Manager;
use project::Project;
use runner::Runner;
use term::Term;
use crate::project::Action;

mod args;
mod manager;
mod project;
mod runner;
mod term;

fn handle_action(action: Action, action_num: usize, job_name: &str) {
    match Runner::run_action(action) {
        Ok(_) => {},
        Err(e) => {
            match e {
                runner::RunnerError::ProgramNotFound(prog) => Term::traceback(job_name, action_num.to_string().as_str(), format!("Cannot find required program: {prog}").as_str()),
                runner::RunnerError::Interrupted => Term::traceback(job_name, action_num.to_string().as_str(), "Program was interrupted"),
                runner::RunnerError::BadExitCode(code) => Term::traceback(job_name, action_num.to_string().as_str(), format!("Program exited with bad exit code: {code}").as_str()),
                runner::RunnerError::DoneButFailed => Term::traceback(job_name, action_num.to_string().as_str(), "Program exited successfully, but failed."),
                runner::RunnerError::Unknown => Term::traceback(job_name, action_num.to_string().as_str(), "Program exited with unknown reason."),
            };
            Term::error("Tesuto finished his work with errors.");
            exit(1)
        }
    }
}

fn main() {
    let args: ArgMatches = args().get_matches();
    match args.subcommand() {
        Some(("new", _sub)) => {
            if Path::new("tesuto.yml").exists() {
                Term::error("Project file already exists.");
                exit(1);
            }

            let new_project: Project = Project::default();
            Manager::write_project(new_project);
            Term::done("Project created. It's saved as `tesuto.yml`.");
        }
        Some(("run", _sub)) => {
            if !Path::new("tesuto.yml").exists() {
                Term::error("Project file not found.");
                exit(1);
            }

            let project = Manager::load_project();
            Term::message(format!("Running project: {}", project.get_name()).as_str());

            if project.is_jobs_empty() {
                Term::message("Nothing to run.");
                exit(0);
            }

            for job in project.get_jobs() {
                for action in job.1.iter().enumerate() {
                    let action_num = action.0 + 1;
                    let job_name = job.0.as_str();
                    handle_action(action.1.clone(), action_num, job_name);
                }
            }
            Term::done("Tesuto finished his work.");
        }
        Some(("run-job", sub)) => {
            if !Path::new("tesuto.yml").exists() {
                Term::error("Project file not found.");
                exit(1);
            }

            if let Some(job) = sub.get_one::<String>("job") {
                if job.is_empty() {
                    Term::error("Job not specified.");
                    exit(1);
                }

                let project = Manager::load_project();
                if !project.is_job_exists(job) {
                    Term::error("Job not found.");
                    exit(1);
                }

                let jobs = project.get_jobs();
                let job_item = jobs.get(job).unwrap();
                for action in job_item.iter().enumerate() {
                    let action_num = action.0 + 1;
                    handle_action(action.1.clone(), action_num, job);
                }
                Term::done("Tesuto finished his work.");
            }
        },
        Some(("list", _sub)) => {
            if !Path::new("tesuto.yml").exists() {
                Term::error("Project file not found.");
                exit(1);
            }

            let project = Manager::load_project();
            if project.is_jobs_empty() {
                Term::warn("This project has no jobs.");
                exit(0);
            }
            Term::message("Jobs in this project:");
            for job in project.get_jobs() {
                Term::no_icon_message(job.0.as_str());
            }
        }
        _ => Term::error("Unknown command."),
    }
}
