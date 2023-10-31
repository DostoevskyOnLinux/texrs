use clap::{arg, Command};

fn cli() -> Command {
    Command::new("texrs")
        .about("Manage a LaTeX project structure.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("new")
                .about("Instantiate new project structure.")
                .arg(arg!(<NAME> "The new project's name."))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("build")
                .about("Build the current LaTeX project.")
                .arg(arg!(<PATH> "Path to the project structure's root."))
                .arg_required_else_help(true),
        )
}

fn main() {
    let matches = cli().get_matches();
    crate::cli::match_command(matches);
}

pub mod build;
pub mod cli;
pub mod config;
pub mod new;
