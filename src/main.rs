use clap::{Arg, Command};
use project::{Manager, Project};
use std::process::exit;

use crate::{project::Stage, runner::Runner};

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
        ])
}

fn main() {
    let args: _ = app().get_matches();
    match args.subcommand() {
        Some(("gen", _sub)) => {
            if Manager::check() {
                println!("Confgiuration file already exists.");
                exit(1);
            }

            println!("Generating new confgiuration...");
            Manager::generate();
            println!("Done! They are saved as 'tesuto.yml'.");
        }
        Some(("run", _sub)) => {
            if !Manager::check() {
                println!("Confgiuration file not exists.");
                exit(1);
            }

            let project: Project = Manager::read();

            println!("Running project '{}'.", project.options.name);
            if project.stages.is_empty() {
                println!("No stages are added to project.");
                exit(1);
            }

            for i in project.stages {
                println!("Running stage: {}", i.name);

                if i.program.is_empty() {
                    println!("No command given. Skipping...");
                    break;
                }

                Runner::run_stage(i);
            }

            println!("All stages passed!");
        }
        _ => println!("Error"),
    }
}
