use clap::{Arg, Command};
use project::{Manager, Project};
use std::process::exit;

use crate::{project::Stage, runner::Runner, term::Term};

mod project;
mod runner;
mod term;

fn app() -> Command {
    Command::new("tesuto")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands([
            Command::new("gen").about("Generate default configuration."),
            Command::new("run").about("Run current testing configuration."),
            Command::new("add").about("Add new stage.")
        ])
}

fn main() {
    let args: _ = app().get_matches();
    match args.subcommand() {
        Some(("gen", _sub)) => {
            if Manager::check() {
                Term::fatal("Confgiuration file already exists.");
                exit(1);
            }

            Term::work("Generating new confgiuration...");
            Manager::generate();
            Term::success("Done! They are saved as 'tesuto.yml'.");
        }
        Some(("run", _sub)) => {
            if !Manager::check() {
                Term::fatal("Confgiuration file not exists.");
                exit(1);
            }

            let project: Project = Manager::read();

            Term::info(&format!("Running project '{}'.", project.options.name));
            if project.stages.is_empty() {
                Term::warn("No stages are added to project.");
                exit(1);
            }

            Term::warn("Option `expectedExitCode` will be ignored, because `expectFail` are true.");
            
            println!("{:?}", project.stages.clone());

            for i in project.stages {
                Term::work(&format!("Running stage: {}", i.name));

                if i.program.is_empty() {
                    Term::warn("No command given. Skipping...");
                    continue;
                }

                Runner::run_stage(i);
            }

            Term::success("All stages passed!");
        }
        Some(("add", _sub)) => {
            if !Manager::check() {
                Term::fatal("Confgiuration file not exists.");
                exit(1);
            }

            let mut project: Project = Manager::read();
            
            let mut new_stage = Stage { ..Default::default() };
            new_stage.name = Term::ask("Name for new stage.", "Stage");
            if project.stages.clone().into_iter().any(|i| i.name == new_stage.name) {
                Term::fatal("Stage with same name already exists.");
                exit(1);
            }

            new_stage.program = Term::ask("Program to run.", "");
            if new_stage.program.is_empty() {
                Term::fatal("No program given.");
                exit(1);
            }

            loop {
                let arg = Term::ask("Argument for program (leave empty to end).", "");
                if arg == "" {
                    break;
                }
                new_stage.args.push(arg);
            }

            project.stages.push(new_stage);
            Manager::write(project);
            Term::success("Stage added!");
            Term::hint("You can change additional options in 'tesuto.yml'.");
            
        }
        _ => println!("Error"),
    }
}
