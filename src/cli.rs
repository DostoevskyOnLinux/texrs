use crate::{config::*, new};
use clap::{arg, ArgMatches, Command, Parser};
use colored::*;
use std::{io, io::Write};

pub fn match_command(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("new", sub_m)) => {
            let config = generate_config(sub_m);
            new::create_dir_structure(config).expect("Location must be writable.");
        }
        Some(("build", _sub_matches)) => {
            unimplemented!(); // TODO: Make build work!
        }
        _ => unreachable!(),
    }
}

fn generate_config(args: &ArgMatches) -> ProjectConfig {
    let mut config = ProjectConfig::new();
    let mut input = String::new();

    let name = args.get_one::<String>("NAME").expect("required").to_owned();
    config.set_name(&name);

    // Select doctype.
    println!("Please select your document type: [article (0), book (1), or letter (2)]");
    print!("{} ", ">".to_string().bold().green());
    let _ = io::stdout().flush();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(_) => {
            eprintln!("Invalid input; continuing!");
        }
    }
    input = input.trim().to_string();
    match &input as &str {
        "0" => {
            config.set_doctype(DocumentType::Article);
            println!("Selecting {}.", "article".blue());
        }
        "1" => {
            config.set_doctype(DocumentType::Book);
            println!("Selecting {}", "book".blue());
        }
        "2" => {
            config.set_doctype(DocumentType::Letter);
            println!("Selecting {}", "letter".blue());
        }
        _ => {
            println!("Selecting letter."); // Is the default.
        }
    }

    // Select driver.
    println!("Please select your driver: [pdflatex (0), xelatex (1), or lualatex (2)]");
    print!("{}", ">".to_string().bold().green());
    let _ = io::stdout().flush();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(_) => {
            eprintln!("Invalid input; continuing!");
        }
    }
    input = input.trim().to_string();
    match &input as &str {
        "0" => config.set_driver("pdflatex"),
        "1" => config.set_driver("xelatex"),
        "2" => config.set_driver("lualatex"),
        _ => {
            println!("Selecting pdflatex."); // Is the default.
        }
    }

    // Choose citations.
    print!("Will this document include citations? (Y/n) ");
    let _ = io::stdout().flush();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(_) => {
            eprintln!("Invalid input; continuing!");
        }
    }
    input = input.trim().to_string();
    match &input as &str {
        "Y" | "y" => config.set_citations(true),
        "N" | "n" => config.set_citations(false),
        _ => {
            println!("Excluding citations."); // Is the default.
        }
    }

    // Choose graphics.
    print!("Will this document include graphics? (Y/n) ");
    let _ = io::stdout().flush();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(_) => {
            eprintln!("Invalid input; continuing!");
        }
    }
    input = input.trim().to_string();
    match &input as &str {
        "Y" | "y" => config.set_graphics(true),
        "N" | "n" => config.set_graphics(false),
        _ => {
            println!("Excluding graphics."); // Is the default.
        }
    }

    config
}
