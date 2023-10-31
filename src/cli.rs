use crate::{config::*, new};
use clap::ArgMatches;
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

/// The following function generates a ProjectConfig
/// struct by asking the user to select what options
/// they need in their document. The ProjectConfig
/// struct can then be processed to achieve the des-
/// ired result, or written to a config.toml file
/// with the serde and toml crates.
/// ## Usage
/// ```rust
/// let config = generate_config(args);
/// let project_name = config.get_name();
///
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
    //println!("INPUT WAS ---{}---", input);
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

    input = String::new();
    // Select driver.
    println!("Please select your driver: [pdflatex (0), xelatex (1), or lualatex (2)]");
    print!("{} ", ">".to_string().bold().green());
    let _ = io::stdout().flush();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(_) => {
            eprintln!("Invalid input; continuing!");
        }
    }
    input = input.trim().to_string();
    //println!("INPUT WAS ---{}---", input);
    match &input as &str {
        "0" => {
            config.set_driver("pdflatex");
            println!("Selecting {}", "pdflatex".green());
        }
        "1" => {
            config.set_driver("xelatex");
            println!("Selecting {}", "xelatex".green());
        }
        "2" => {
            config.set_driver("lualatex");
            println!("Selecting {}", "lualatex".green());
        }
        _ => {
            println!("Selecting pdflatex."); // Is the default.
        }
    }

    input = String::new();
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
    //println!("INPUT WAS ---{}---", input);
    match &input as &str {
        "Y" | "y" => {
            config.set_citations(true);
            println!("Setting citations to {}", "true".blue());
        }
        "N" | "n" => {
            config.set_citations(false);
            println!("Setting citations to {}", "false".red());
        }
        _ => {
            println!("Including citations."); // Is the default.
        }
    }

    input = String::new();
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
    //println!("INPUT WAS ---{}---", input);
    match &input as &str {
        "Y" | "y" => config.set_graphics(true),
        "N" | "n" => config.set_graphics(false),
        _ => {
            println!("Including graphics."); // Is the default.
        }
    }

    config
}
