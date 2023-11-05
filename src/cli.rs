// + - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - +
// | This file is part of texrs.                                                                                                       |
// |                                                                                                                                   |
// | texrs is free software: you can redistribute it and/or modify it under the terms                                                  |
// | of the GNU General Public License as published by the Free Software Foundation,                                                   |
// | either version 3 of the License, or (at your option) any later version.                                                           |
// |                                                                                                                                   |
// | texrs is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;                                                |
// | without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.                                         |
// | See the GNU General Public License for more details.                                                                              |
// |                                                                                                                                   |
// | You should have received a copy of the GNU General Public License along with texrs. If not, see <https://www.gnu.org/licenses/>.  |
// + - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - +
// | Copyright (c) 2023 Ethan Barry <ethanbarry@howdytx.net>                                                                           |
// | Feel free to contact the author if you do come across this source code for some reason...                                         |
// | <https://github.com/DostoevskyOnLinux> is the author's profile.                                                                   |
// + - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - +

use crate::{build::*, config::*, new};
use clap::ArgMatches;
use colored::*;
use std::{io, io::Write};

pub fn match_command(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("new", sub_m)) => {
            let config = generate_config(sub_m);
            new::create_dir_structure(config).expect("Location must be writable.");
        }
        Some(("build", sub_m)) => {
            let config = read_config(sub_m.clone())
                .expect("Config file must be present in the directory given.");
            build_from_config(config).expect("Location must be writable.");
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

/* TODO: Revamp this CLI and add some conditions to make defaults better.
I should be able to select my doctype and have everything else be OK. */
fn generate_config(args: &ArgMatches) -> ProjectConfig {
    let mut config = ProjectConfig::new();
    let mut input: String;

    let name = args.get_one::<String>("NAME").expect("required").to_owned();
    config.set_name(&name);

    // Select doctype.
    loop {
        input = String::new();
        print!(
            "{} [(a)rticle, (b)ook, or (L)etter]: ",
            "Select template.".blue()
        );
        let _ = io::stdout().flush();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => {
                eprintln!("Invalid input; continuing!");
            }
        }
        input = input.trim().to_string();

        match &input as &str {
            "A" | "a" => {
                config.set_doctype(DocumentType::Article);
                println!("Selecting {}.", "article".green());
                break;
            }
            "B" | "b" => {
                config.set_doctype(DocumentType::Book);
                println!("Selecting {}", "book".green());
                break;
            }
            "L" | "l" => {
                config.set_doctype(DocumentType::Letter);
                println!("Selecting {}", "letter".green());
                break;
            }
            _ => {
                println!("Selecting {}.", "letter".green()); // Is the default.
                break;
            }
        }
    }

    loop {
        input = String::new();
        // Select driver.
        print!(
            "{} [(P)dflatex, (x)elatex, or (l)ualatex]: ",
            "Select driver.".blue()
        );
        let _ = io::stdout().flush();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => {
                eprintln!("Invalid input; continuing!");
            }
        }
        input = input.trim().to_string();

        match &input as &str {
            "P" | "p" => {
                config.set_driver("pdflatex");
                println!("Selecting {}", "pdflatex".green());
                break;
            }
            "X" | "x" => {
                config.set_driver("xelatex");
                println!("Selecting {}", "xelatex".green());
                break;
            }
            "L" | "l" => {
                config.set_driver("lualatex");
                println!("Selecting {}", "lualatex".green());
                break;
            }
            _ => {
                println!("Selecting pdflatex."); // Is the default.
                break;
            }
        }
    }

    loop {
        input = String::new();
        // Choose citations.
        print!("{} (Y/n) ", "Include citations?".blue());
        let _ = io::stdout().flush();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => {
                eprintln!("Invalid input; continuing!");
            }
        }
        input = input.trim().to_string();

        match &input as &str {
            "Y" | "y" => {
                config.set_citations(true);
                println!("Setting citations to {}", "true".green());
                break;
            }
            "N" | "n" => {
                config.set_citations(false);
                println!("Setting citations to {}", "false".green());
                break;
            }
            _ => {
                println!("Setting citations to {}", "true".green()); // Is the default.
                break;
            }
        }
    }

    loop {
        input = String::new();
        // Choose graphics.
        print!("{} (Y/n) ", "Include graphics?".blue());
        let _ = io::stdout().flush();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => {
                eprintln!("Invalid input; continuing!");
            }
        }
        input = input.trim().to_string();

        match &input as &str {
            "Y" | "y" => {
                config.set_graphics(true);
                println!("Setting graphics to {}", "true".green());
                break;
            }
            "N" | "n" => {
                config.set_graphics(false);
                println!("Setting graphics to {}", "false".green());
                break;
            }
            _ => {
                println!("Setting graphics to {}", "true".green()); // Is the default.
                break;
            }
        }
    }
    config
}
