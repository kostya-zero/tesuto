use std::{fs, path::Path, process::exit};

use args::args;
use clap::ArgMatches;
use project::Project;
use runner::Runner;
use term::Term;

mod args;
mod platform;
mod project;
mod runner;
mod term;

fn load_project(path: &str) -> Option<Project> {
    if !Path::new(path).exists() {
        Term::error("Project file not found.");
        exit(1);
    }

    let json_string: String = fs::read_to_string(path).unwrap();

    match Project::from(json_string.as_str()) {
        Ok(i) => Some(i),
        Err(_) => {
            Term::error("Project file has a bad structure.");
            exit(1);
        }
    }
}

fn main() {
    let args: ArgMatches = args().get_matches();
    let project_path = args.get_one::<String>("project").unwrap();

    match args.subcommand() {
        Some(("new", sub)) => {
            if Path::new(project_path).exists() {
                Term::error("Project file already exists.");
                exit(1);
            }
            let project_name: String = sub
                .get_one::<String>("name")
                .filter(|name| !name.is_empty())
                .cloned()
                .unwrap_or_else(|| {
                    Term::input("How you want to name your project?", "TesutoProject")
                });

            let use_example = if sub.get_flag("example") {
                true
            } else {
                Term::ask("Do you want to use example project?", false)
            };

            let new_project: Project = if use_example {
                Project::example(project_name.as_str())
            } else {
                Project::new(project_name.as_str())
            };
            match serde_yaml::to_string(&new_project) {
                Ok(project_string) => match fs::write(project_path, project_string) {
                    Ok(_) => Term::done("Project created. It's saved as `tesuto.yml`."),
                    Err(_) => Term::fail("Failed to write content to file."),
                },
                Err(_) => Term::fail("Failed to convert structure to string."),
            }
        }
        Some(("check", _sub)) => {
            Term::work("Trying to load configuration...");
            if load_project(project_path).is_some() {
                Term::done("Your project is OK.");
                exit(0);
            }
        }
        Some(("run", _sub)) => {
            let project = load_project(project_path).unwrap();
            Term::message(format!("Running project: {}", project.get_name()).as_str());

            if project.is_jobs_empty() {
                Term::message("Nothing to run.");
                exit(0);
            }

            let runner = Runner::new(project);
            if let Err(error) = runner.run_project() {
                Term::fail(error.to_string().as_str());
            }

            Term::done("Tesuto finished his work.");
        }
        Some(("run-job", sub)) => {
            if let Some(job) = sub.get_one::<String>("job") {
                if job.is_empty() {
                    Term::error("Job not specified.");
                    exit(1);
                }

                let project = load_project(project_path).unwrap();
                if !project.is_job_exists(job) {
                    Term::error("Job not found.");
                    exit(1);
                }

                let jobs = project.get_jobs();
                let job_item = jobs.get(job).unwrap();
                Term::work(format!("Job '{}'.", job).as_str());
                let runner = Runner::new(project);

                if let Err(error) = runner.run_job((job, job_item)) {
                    Term::fail(error.to_string().as_str());
                }
                Term::done("Tesuto finished his work.");
            }
        }
        Some(("list", _sub)) => {
            let project = load_project(project_path).unwrap();
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
