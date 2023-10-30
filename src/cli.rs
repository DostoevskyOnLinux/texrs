use clap::{arg, Command, Parser, ArgMatches};
use colored::*;

pub fn match_command(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("new", sub_m)) => {
            println!(
                "Creating {}...",
                sub_m.get_one::<String>("PROJECT").expect("required").to_string().blue().bold()
            );
            let _ = new::new(sub_matches.get_one::<String>("PROJECT").expect("required").to_string(), new::PType::Article);
        }
        Some(("build", _sub_matches)) => {
            match build::build() {
                Ok(name) => { println!("Building {}...", name.blue()) },
                Err(error) => { eprintln!("{}", error.to_string().red()) }
            }
        }
        _ => unreachable!(),
    }
}
