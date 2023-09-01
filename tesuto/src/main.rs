use std::{path::Path, process::exit};

use args::args;
use clap::ArgMatches;
use manager::Manager;
use runner::Runner;
use term::Term;
use tesuto_project::{Project, Stage, Stages};

mod args;
mod manager;
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
            Term::message(format!("Running project: {}", project.get_project_name()).as_str());
            let stages: Stages = project.get_stages();

            if stages.is_empty() {
                Term::message("Nothing to run...");
                exit(0);
            }

            for (k, v) in stages {
                Term::message(format!("Current stage: {}", k).as_str());
                match Runner::run_stage(v) {
                    true => {
                        Term::done("Stage passed.");
                    }
                    false => {
                        Term::error("Aborting procedure...");
                        exit(1);
                    }
                }
            }
        }
        Some(("list", _sub)) => {
            if !Path::new("tesuto.yml").exists() {
                Term::error("Project file not found.");
                exit(1);
            }

            let project = Manager::load_project();
            Term::message("Stages in this project:");
            for (k, _) in project.get_stages().iter() {
                Term::no_icon_message(k.as_str());
            }
        }
        Some(("add", sub)) => {
            if !Path::new("tesuto.yml").exists() {
                Term::error("Project file not found.");
                exit(1);
            }

            let name: &str = sub.get_one::<String>("name").unwrap();
            let mut project: Project = Manager::load_project();
            let stages: Stages = project.get_stages();
            if stages.contains_key(name) {
                Term::error("Stage with same name already exists.");
                exit(1);
            }

            project.add_stage(name, Stage::default());
            Manager::write_project(project);
            Term::done("Stage added.");
        }
        _ => Term::error("Unknown command."),
    }
}
