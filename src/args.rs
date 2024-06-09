use clap::{value_parser, Arg, ArgAction, Command};

pub fn args() -> Command {
    Command::new("tesuto")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("Konstantin Zhigaylo <zero@kostyazero.com>")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .arg(
            Arg::new("project")
                .help("Path to the project file.")
                .default_value("tesuto.yml")
                .hide_default_value(true)
                .value_parser(clap::value_parser!(String))
                .required(false)
                .num_args(1),
        )
        .subcommands([
            Command::new("new").about("Create new project.").args([
                Arg::new("name")
                    .help("Name for your project.")
                    .short('n')
                    .long("name")
                    .required(false)
                    .default_value("")
                    .value_parser(value_parser!(String)),
                Arg::new("example")
                    .help("Use example project.")
                    .short('e')
                    .long("example")
                    .required(false)
                    .action(ArgAction::SetTrue),
            ]),
            Command::new("check").about("Check if project is OK."),
            Command::new("run").about("Run project configuration."),
            Command::new("run-job")
                .about("Run specific job in project.")
                .arg(
                    Arg::new("job")
                        .help("Name of the job to run.")
                        .num_args(1)
                        .required(false)
                        .value_parser(clap::value_parser!(String))
                        .default_value(""),
                ),
            Command::new("list").about("List all stages in project."),
        ])
}
