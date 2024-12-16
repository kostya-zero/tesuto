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
                .long("project")
                .short('p')
                .help("Path to the project file.")
                .default_value("tesuto.yml")
                .hide_default_value(true)
                .value_parser(clap::value_parser!(String))
                .required(false)
                .global(true)
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
            Command::new("run").about("Run project configuration.").arg(
                Arg::new("job")
                    .required(false)
                    .help("Name of the job to run.")
                    .short('j')
                    .long("job")
                    .value_parser(value_parser!(String)),
            ),
            Command::new("list").about("List all stages in project."),
        ])
}
