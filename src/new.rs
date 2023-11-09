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

use crate::{config::*, DocumentType};
use colored::*;
use std::io;
use std::io::Write;
use std::process::Command;
use std::{fs, fs::File};

const ARTICLE_TEMPLATE: &str = include_str!("../res/article.tex");
const LETTER_TEMPLATE: &str = include_str!("../res/letter.tex");
const BIBTEX_TEMPLATE: &str = include_str!("../res/refs.bib");

// TODO: Remove .expect() in these functions.
fn initialize_git_repository(config: ProjectConfig) -> Result<(), io::Error> {
    let project_directory = config.get_name();

    let mut git_init = Command::new("git");
    let git_init_output = git_init
        .args(&["init"])
        .current_dir(project_directory.clone());
    let output = git_init_output.output().expect("Git failed to initialize.");

    if output.status.success() {
        println!(
            "Git repository initialized successfully in {}.",
            project_directory.blue()
        );
    } else {
        eprintln!(
            "Failed to initialize Git repository:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}

fn add_to_git_repository(config: ProjectConfig) -> Result<(), io::Error> {
    let project_directory = config.get_name();

    let mut git_add = Command::new("git");
    let git_add_output = git_add
        .args(&["add", "."])
        .current_dir(project_directory.clone());
    let output = git_add_output.output().expect("Git failed to add files.");

    if output.status.success() {
        println!(
            "Git repository populated successfully in {}.",
            project_directory.blue()
        );
    } else {
        eprintln!(
            "Failed to populate Git repository:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let mut git_commit = Command::new("git");
    let git_commit_output = git_commit
        .args(&["commit", "-m", "\"Initialize repository.\""])
        .current_dir(project_directory.clone());
    let output = git_commit_output
        .output()
        .expect("Git failed to commit files.");

    if output.status.success() {
        println!(
            "Git repository committed successfully in {}.",
            project_directory.blue()
        );
    } else {
        eprintln!(
            "Failed to commit Git repository:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}

/* -------------------------------------------------------------------- */
/// This method creates a directory structure based on a ProjectConfig
/// struct passed in by the caller. It returns a result depending on the
/// file IO, which may experience an issue.
///
/// ## Usage
///
/// ```rust
/// create_structure(config).expect("File IO failed.");
/// ```
/* -------------------------------------------------------------------- */
pub fn create_structure(config: ProjectConfig) -> Result<(), io::Error> {
    let name = config.get_name();
    match fs::create_dir(name.clone()) {
        Ok(_) => println!("Created {} directory.", name.blue()),
        Err(err) => eprintln!("{}", err),
    }

    // Check for graphics in the project.
    match config.get_graphics() {
        true => match fs::create_dir(name.clone() + "/graphics") {
            Ok(_) => println!("Created {} directory.", "graphics".to_owned().blue()),
            Err(err) => eprintln!("{}", err),
        },
        false => println!("Skipping {} directory.", "graphics".to_owned().yellow()),
    }

    match config.get_citations() {
        true => match fs::create_dir(name.clone() + "/bib") {
            Ok(_) => {
                println!("Created {} directory.", "bib".to_owned().blue());
                match File::create(name.clone() + "/bib/refs.bib") {
                    Ok(mut file) => match file.write_all(BIBTEX_TEMPLATE.as_bytes()) {
                        Ok(_) => println!("Created {} file.", "refs.bib".blue()),
                        Err(err) => eprintln!("{}", err),
                    },
                    Err(err) => eprintln!("{}", err),
                };
            }
            Err(err) => eprintln!("{}", err),
        },
        false => println!("Skipping {} directory.", "bib".to_owned().yellow()),
    }

    match config.get_doctype() {
        DocumentType::Article => match fs::create_dir(name.clone() + "/tex") {
            Ok(_) => {
                println!("Created {} directory.", "tex".to_owned().blue());
                match File::create(name.clone() + "/tex/" + &name + ".tex") {
                    Ok(mut file) => match file.write_all(ARTICLE_TEMPLATE.as_bytes()) {
                        Ok(_) => println!("Created {} file.", (name.clone() + ".tex").blue()),
                        Err(err) => eprintln!("{}", err),
                    },
                    Err(err) => eprintln!("{}", err),
                }
            }
            Err(err) => eprintln!("{}", err),
        },
        DocumentType::Book => match fs::create_dir(name.clone() + "/tex") {
            Ok(_) => {
                println!("Created {} directory.", "tex".to_owned().blue());
                match File::create(name.clone() + "/tex/" + &name + ".tex") {
                    Ok(mut file) => match file.write_all(ARTICLE_TEMPLATE.as_bytes()) {
                        Ok(_) => println!("Created {} file.", (name.clone() + ".tex").blue()),
                        Err(err) => eprintln!("{}", err),
                    },
                    Err(err) => eprintln!("{}", err),
                }
            }
            Err(err) => eprintln!("{}", err),
        },
        DocumentType::Thesis => match fs::create_dir(name.clone() + "/tex") {
            Ok(_) => {
                println!("Created {} directory.", "tex".to_owned().blue());
                match File::create(name.clone() + "/tex/" + &name + ".tex") {
                    Ok(mut file) => match file.write_all(ARTICLE_TEMPLATE.as_bytes()) {
                        Ok(_) => println!("Created {} file.", (name.clone() + ".tex").blue()),
                        Err(err) => eprintln!("{}", err),
                    },
                    Err(err) => eprintln!("{}", err),
                }
            }
            Err(err) => eprintln!("{}", err),
        },
        DocumentType::Presentation => match fs::create_dir(name.clone() + "/tex") {
            Ok(_) => {
                println!("Created {} directory.", "tex".to_owned().blue());
                match File::create(name.clone() + "/tex/" + &name + ".tex") {
                    Ok(mut file) => match file.write_all(ARTICLE_TEMPLATE.as_bytes()) {
                        Ok(_) => println!("Created {} file.", (name.clone() + ".tex").blue()),
                        Err(err) => eprintln!("{}", err),
                    },
                    Err(err) => eprintln!("{}", err),
                }
            }
            Err(err) => eprintln!("{}", err),
        },
        DocumentType::MathArticle => match fs::create_dir(name.clone() + "/tex") {
            Ok(_) => {
                println!("Created {} directory.", "tex".to_owned().blue());
                match File::create(name.clone() + "/tex/" + &name + ".tex") {
                    Ok(mut file) => match file.write_all(ARTICLE_TEMPLATE.as_bytes()) {
                        Ok(_) => println!("Created {} file.", (name.clone() + ".tex").blue()),
                        Err(err) => eprintln!("{}", err),
                    },
                    Err(err) => eprintln!("{}", err),
                }
            }
            Err(err) => eprintln!("{}", err),
        },
        DocumentType::Letter => match fs::create_dir(name.clone() + "/tex") {
            Ok(_) => {
                println!("Created {} directory.", "tex".to_owned().blue());
                match File::create(name.clone() + "/tex/" + &name + ".tex") {
                    Ok(mut file) => match file.write_all(LETTER_TEMPLATE.as_bytes()) {
                        Ok(_) => println!("Created {} file.", (name.clone() + ".tex").blue()),
                        Err(err) => eprintln!("{}", err),
                    },
                    Err(err) => eprintln!("{}", err),
                }
            }
            Err(err) => eprintln!("{}", err),
        },
    }

    match initialize_git_repository(config.clone()) {
        Ok(_) => match add_to_git_repository(config.clone()) {
            Ok(_) => println!("Created {} repository.", "git".to_owned().blue()),
            Err(err) => eprintln!("{}", err),
        },
        Err(err) => eprintln!("{}", err),
    }

    Ok(())
}
