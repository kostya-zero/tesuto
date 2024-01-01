use std::{path::Path, process::exit};

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
                    // if !Runner::run_action(action) {
                    //     Term::error(format!("Job `{}` failed to run. Exiting...", job.0).as_str());
                    //     exit(1);
                    // }
                    match Runner::run_action(action.1.clone()) {
                        Ok(_) => {},
                        Err(e) => {
                            let action_num = action.0 + 1;
                            let job_name = job.0.as_str();
                            match e {
                                runner::RunnerError::ProgramNotFound(prog) => Term::traceback(job_name, action_num.to_string().as_str(), format!("Cannot find required program: {prog}").as_str()),
                                runner::RunnerError::Interrupted => Term::traceback(job_name, action_num.to_string().as_str(), format!("Program was interrupted").as_str()),
                                runner::RunnerError::BadExitCode(code) => Term::traceback(job_name, action_num.to_string().as_str(), format!("Program exited with bad exit code: {code}").as_str()),
                                runner::RunnerError::DoneButFailed => Term::traceback(job_name, action_num.to_string().as_str(), format!("Program exited successfully, but failed.").as_str()),
                                runner::RunnerError::Unknown => Term::traceback(job_name, action_num.to_string().as_str(), format!("Program exited with unknown reason.").as_str()),
                            };
                            Term::error("Tesuto finished his work with errors.");
                            exit(1)
                        }
                    }
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
                for action in job_item {
                    // if !Runner::run_action(action.clone()) {
                    //     Term::error(format!("Job `{}` failed to run. Exiting...", job).as_str());
                    //     exit(1);
                    // }
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
