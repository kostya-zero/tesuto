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
                for action in job.1 {
                    if !Runner::run_action(action) {
                        Term::error(format!("Job `{}` failed to run. Exiting...", job.0).as_str());
                        exit(1);
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
                    if !Runner::run_action(action.clone()) {
                        Term::error(format!("Job `{}` failed to run. Exiting...", job).as_str());
                        exit(1);
                    }
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
            Term::message("Stages in this project:");
        }
        _ => Term::error("Unknown command."),
    }
}
