use std::{fs, path::Path, process::exit};

use crate::project::Action;
use args::args;
use clap::ArgMatches;
use manager::Manager;
use project::Project;
use runner::Runner;
use term::Term;

mod args;
mod manager;
mod project;
mod runner;
mod term;

fn handle_action(action: Action, job_name: &str) {
    match Runner::run_action(action) {
        Ok(_) => {}
        Err(e) => {
            match e {
                runner::RunnerError::ProgramNotFound(prog) => Term::traceback(
                    job_name,
                    format!("Cannot find required program: {prog}").as_str(),
                ),
                runner::RunnerError::Interrupted => {
                    Term::traceback(job_name, "Program was interrupted")
                }
                runner::RunnerError::BadExitCode(code) => Term::traceback(
                    job_name,
                    format!("Program exited with bad exit code: {code}").as_str(),
                ),
                runner::RunnerError::DoneButFailed => {
                    Term::traceback(job_name, "Program exited successfully, but failed.")
                }
                runner::RunnerError::Unknown => {
                    Term::traceback(job_name, "Program exited with unknown reason.")
                }
            };
            Term::error("Tesuto finished his work with errors.");
            exit(1)
        }
    }
}

fn load_project() -> Option<Project> {
    match Manager::load_project() {
        Ok(i) => Some(i),
        Err(e) => match e {
            manager::ManagerError::ReadError => {
                Term::fail("Failed to read configuration file.");
                None
            }
            manager::ManagerError::BadStructure => {
                Term::fail("Configuration file is bad structured.");
                None
            }
            _ => {
                Term::fail("Unexpected error occured.");
                None
            }
        },
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
            match serde_yaml::to_string(&new_project) {
                Ok(project_string) => match fs::write("tesuto.yml", project_string) {
                    Ok(_) => Term::done("Project created. It's saved as `tesuto.yml`."),
                    Err(_) => Term::fail("Failed to write file to "),
                },
                Err(_) => Term::fail("Failed to convert structure to string."),
            }
        }
        Some(("check", _sub)) => {
            if !Path::new("tesuto.yml").exists() {
                Term::error("Project file not found.");
                exit(1);
            }

            Term::work("Trying to load confgiuration...");
            let project = load_project().unwrap();
        }
        Some(("run", _sub)) => {
            if !Path::new("tesuto.yml").exists() {
                Term::error("Project file not found.");
                exit(1);
            }

            let project = load_project().unwrap();
            Term::message(format!("Running project: {}", project.get_name()).as_str());

            if project.is_jobs_empty() {
                Term::message("Nothing to run.");
                exit(0);
            }

            for job in project.get_jobs() {
                Term::work(format!("Working on the job {}...", job.0).as_str());
                for action in job.1.iter() {
                    let job_name = job.0.as_str();
                    handle_action(action.clone(), job_name);
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

                let project = load_project().unwrap();
                if !project.is_job_exists(job) {
                    Term::error("Job not found.");
                    exit(1);
                }

                let jobs = project.get_jobs();
                let job_item = jobs.get(job).unwrap();
                Term::work(format!("Working on the job {}...", job).as_str());
                for action in job_item.iter().enumerate() {
                    handle_action(action.1.clone(), job);
                }
                Term::done("Tesuto finished his work.");
            }
        }
        Some(("list", _sub)) => {
            if !Path::new("tesuto.yml").exists() {
                Term::error("Project file not found.");
                exit(1);
            }

            let project = load_project().unwrap();
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
