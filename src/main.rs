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
                Term::message("Nothing to run...");
                exit(0);
            }

            for job in project.get_jobs() {
                for action in job.1 {
                    if !action.is_name_empty() {
                        Term::message(action.get_name().as_str());
                    } else if !action.is_program_empty() {
                        Term::message(format!("Running `{}`", action.get_program()).as_str());
                    }

                    if !action.is_program_empty() {
                        let command = action.get_program();
                        let mut args: Vec<String> = Vec::new();
                        if !action.is_args_empty() {
                            args = action.get_args();
                        }
                        if !Runner::run_command(command.as_str(), args) {
                            Term::error("Job failed. Aborting procedure...");
                            exit(1);
                        }
                    }
                }
            }
            Term::done("Tesuto finished his work.");
        }
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
