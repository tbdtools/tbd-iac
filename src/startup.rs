use clap::{arg, Command};

pub fn run() -> Command {
    Command::new("tbdiac")
        .about("TBD infrastructure as code")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("new")
                .about("Create a new project")
                .arg(
                    arg!(<name> "The name of the project")
                        .short('n')
                        .long("name"),
                )
                .arg(
                    arg!(<path> "The path to create the project in")
                        .short('p')
                        .long("path"),
                )
                .arg_required_else_help(true),
        )
}
