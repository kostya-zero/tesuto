use clap::{Arg, Command};

pub fn args() -> Command {
    Command::new("tesuto")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("Konstantin Zhigaylo <zero@kostyazero.com>")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommands([
            Command::new("new").about("Create new project."),
            Command::new("run").about("Run project configuration."),
            Command::new("run-job").about("Run specific job in project.").arg(
                Arg::new("job")
                    .help("Name of the job to run.")
                    .num_args(1)
                    .required(false)
                    .value_parser(clap::value_parser!(String))
                    .default_value("")
            ),
            Command::new("list").about("List all stages in project."),
        ])
}
