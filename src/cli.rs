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
// | Copyright (c) 2024 Ethan Barry <ethanbarry@howdytx.net>                                                                           |
// | Feel free to contact the author if you do come across this source code for some reason...                                         |
// | <https://github.com/ethanbarry> is the author's profile.                                                                          |
// + - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - +

use crate::{config::*, DocumentType};
use colored::*;
use std::{io, io::Write};

/// Prompt for yes/no values and return an Option<bool>.
// TODO Extract to its own crate and publish on crates.io.
pub fn prompt_tf(prompt: &str, colored: &str, color: &str) -> Option<bool> {
    let mut input = String::new();
    input.clear();
    match color {
        "blue" => print!(
            "{}? {} (Y/n): ",
            prompt.to_owned().trim(),
            colored.trim().to_owned().blue()
        ),
        "red" => print!(
            "{}? {} (Y/n): ",
            prompt.to_owned().trim(),
            colored.trim().to_owned().red()
        ),
        "green" => print!(
            "{}? {} (Y/n): ",
            prompt.to_owned().trim(),
            colored.trim().to_owned().green()
        ),
        "yellow" => print!(
            "{}? {} (Y/n): ",
            prompt.to_owned().trim(),
            colored.trim().to_owned().yellow()
        ),
        _ => print!("{} (Y/n): ", prompt.to_owned().trim()),
    }
    let _ = io::stdout().flush();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(_) => {
            eprintln!("Invalid input; continuing!");
        }
    }
    input = input.trim().to_string();
    match &input as &str {
        "Y" | "y" => Some(true),
        "N" | "n" => Some(false),
        _ => None,
    }
}

fn prompt_selection(prompt: &str, colored: &str, color: &str) -> Option<String> {
    let mut input = String::new();
    input.clear();
    match color {
        "blue" => print!(
            "{}: [{}]: ",
            prompt.trim().to_owned(),
            colored.trim().to_owned().blue()
        ),
        "red" => print!(
            "{}: [{}]: ",
            prompt.trim().to_owned(),
            colored.trim().to_owned().red()
        ),
        "green" => print!(
            "{}: [{}]: ",
            prompt.trim().to_owned(),
            colored.trim().to_owned().green()
        ),
        "yellow" => print!(
            "{}: [{}]: ",
            prompt.trim().to_owned(),
            colored.trim().to_owned().yellow()
        ),
        _ => print!(
            "{}: [{}]: ",
            prompt.trim().to_owned(),
            colored.trim().to_owned()
        ),
    }
    let _ = io::stdout().flush();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(_) => {
            eprintln!("Invalid input; continuing!");
        }
    }
    input = input.trim().to_string();
    Some(input)
}

// TODO: Restructure menu after adding choices to the templates.
// | <Select Driver>
// | <Select Document Category>
// | <Common> | <Mathematical> | <Formal> | <Personal> | <Technical>
// TODO: Make graphics and citations dependent on the document type.
// | <Select Citations>
// | <Select Graphics>
pub fn config_menu(name: &str) -> ProjectConfig {
    let mut config = ProjectConfig::new();

    config.set_name(name);

    // Prompt for driver:
    match prompt_selection(
        "Select driver",
        "(P)dflatex, (x)elatex, (l)ualatex",
        "green",
    )
    .unwrap_or_default()
    .as_str()
    {
        "P" | "p" => config.set_driver("pdflatex"),
        "X" | "x" => config.set_driver("xelatex"),
        "L" | "l" => config.set_driver("lualatex"),
        _ => config.set_driver("pdflatex"),
    }

    // Prompt for DocumentType:
    match prompt_selection(
        "Select document type",
        "(A)rticle, (b)ook, (l)etter, (m)athematical article, (n)otes, (p)resentation, (r)ecipe, (t)hesis",
        "green",
    )
    .unwrap_or_default()
    .as_str()
    {
        "A" | "a" => config.set_doctype(DocumentType::Article),
        "B" | "b" => config.set_doctype(DocumentType::Book),
        "L" | "l" => config.set_doctype(DocumentType::Letter),
        "M" | "m" => config.set_doctype(DocumentType::MathArticle),
        "N" | "n" => config.set_doctype(DocumentType::Notes),
        "P" | "p" => config.set_doctype(DocumentType::Presentation),
        "R" | "r" => config.set_doctype(DocumentType::Recipe),
        "T" | "t" => config.set_doctype(DocumentType::Thesis),
        _ => config.set_doctype(DocumentType::Article),
    }

    // Prompt for citations:
    match prompt_tf("Include citations", "", "green") {
        Some(val) => {
            if val {
                config.set_citations(val);
            } else {
                config.set_citations(false);
            }
        }
        None => {
            config.set_citations(true);
        }
    }

    // Prompt for graphics:
    match prompt_tf("Include graphics", "", "green") {
        Some(val) => {
            if val {
                config.set_graphics(val);
            } else {
                config.set_graphics(false);
            }
        }
        None => {
            config.set_graphics(true);
        }
    }

    config
}
