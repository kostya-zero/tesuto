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
        Some(("run", sub)) => {
            if !Path::new("tesuto.yml").exists() {
                Term::error("Project file not found.");
                exit(1);
            }

            let project = Manager::load_project();
            let mut stages: Stages = HashMap::new();
            let scenario: &str = sub.get_one::<String>("scenario").unwrap();
            if scenario.is_empty() {
                for i in project.get_stages().iter() {
                    stages.insert(i.0.clone(), i.1.clone());
                }
            } else if project.scenario_exists(scenario) {
                let scenario_info: Vec<String> = project.get_scenario(scenario);
                for i in project.get_stages().iter() {
                    if scenario_info.contains(i.0) {
                        stages.insert(i.0.clone(), i.1.clone());
                    }
                }
            }
            Term::message(format!("Running project: {}", project.get_project_name()).as_str());

            if stages.is_empty() {
                Term::message("Nothing to run...");
                exit(0);
            }

            for (k, v) in stages.iter() {
                Term::message(format!("Current stage: {}", k).as_str());
                match Runner::run_stage(v.clone()) {
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

            let stage = project.get_stage(name);

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
        Some(("add-scenario", sub)) => {
            if !Path::new("tesuto.yml").exists() {
                Term::error("Project file not found.");
                exit(1);
            }

            let name: &str = sub.get_one::<String>("name").unwrap();
            let mut project: Project = Manager::load_project();
            if project.scenario_exists(name) {
                Term::error("Scenario with same name already exists.");
                exit(1);
            }

            project.add_scenario(name);
            Manager::write_project(project);
            Term::done("Scenario added.");
        }
        _ => Term::error("Unknown command."),
    }
}
