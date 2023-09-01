use clap::{Arg, Command};

pub fn args() -> Command {
    Command::new("tesuto")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("Konstantin Zhigaylo <kostya.zero@outlook.com>")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommands([
            Command::new("new").about("Create new project."),
            Command::new("run").about("Run project configuration."),
            Command::new("run-stage").about("Run specific stage.").arg(
                Arg::new("stage")
                    .help("Name of stage to run.")
                    .required(true)
                    .num_args(1)
                    .value_parser(clap::value_parser!(String)),
            ),
            Command::new("list").about("List all stages in project."),
            Command::new("add").about("Add new stage to project.").arg(
                Arg::new("name")
                    .help("Name for new stage.")
                    .required(true)
                    .num_args(1)
                    .value_parser(clap::value_parser!(String)),
            ),
        ])
}
