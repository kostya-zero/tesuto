use std::{collections::HashMap, path::Path, process::exit};

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
            let stages: Stages = project.get_stages();
            Term::message(format!("Running project: {}", project.get_project_name()).as_str());

            if stages.is_empty() {
                Term::message("Nothing to run...");
                exit(0);
            }

            for stage in stages.iter() {
                let stage_name: &str = if let Some(name) = &stage.1.get_name() {
                    name
                } else {
                    &stage.0
                };
                Term::message(format!("Current stage: {}", stage_name).as_str());
                match Runner::run_stage(stage.1.clone()) {
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
        Some(("run-stage", sub)) => {
            if !Path::new("tesuto.yml").exists() {
                Term::error("Project file not found.");
                exit(1);
            }

            let name: &str = sub.get_one::<String>("stage").unwrap();
            let project: Project = Manager::load_project();

            if !project.stage_exists(name) {
                Term::error("Stage with given name not found.");
                exit(1);
            }

            let stage: Stage = project.get_stage(name);

            Term::message(format!("Running stage: {}", name).as_str());
            match Runner::run_stage(stage) {
                true => {
                    Term::done("Stage passed.");
                }
                false => {
                    Term::error("Aborting procedure...");
                    exit(1);
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
            for i in project.get_stages().iter() {
                Term::no_icon_message(i.0.as_str());
            }
        }
        Some(("add", sub)) => {
            if !Path::new("tesuto.yml").exists() {
                Term::error("Project file not found.");
                exit(1);
            }

            let name: &str = sub.get_one::<String>("name").unwrap();
            let mut project: Project = Manager::load_project();
            if project.stage_exists(name) {
                Term::error("Stage with same name already exists.");
                exit(1);
            }

            let mut new_stage: Stage = Stage::default();
            new_stage.set_name(name);
            Manager::write_project(project);
            Term::done("Stage added.");
        }
        _ => Term::error("Unknown command."),
    }
}
