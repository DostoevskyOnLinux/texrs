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

use crate::{
    config::{self, *},
    DocumentType,
};
use colored::*;
use std::io;
use std::io::Write;
use std::process::Command;
use std::{fs, fs::File};

const ARTICLE_TEMPLATE: &str = include_str!("../res/article.tex");
const LETTER_TEMPLATE: &str = include_str!("../res/letter.tex");
const BIBTEX_TEMPLATE: &str = include_str!("../res/refs.bib");

pub fn create_dir_structure(config: ProjectConfig) -> Result<(), io::Error> {
    // Always create a new directory.
    let project_name = &config.get_name().to_owned();
    fs::create_dir(project_name.clone()).expect("Location must be writable.");

    match config.get_graphics() {
        // If true, create graphics dir.
        true => {
            fs::create_dir(project_name.clone() + "/graphics").expect("Location must be writable.");
            let msg = String::from("graphics");
            println!("Created {} directory.", msg.blue().bold());
        }
        false => {
            let msg = String::from("No graphics path needed.");
            println!("{}", msg);
        }
    }

    match config.get_citations() {
        true => {
            fs::create_dir(project_name.clone() + "/bib").expect("Location must be writable.");
            let mut refs = File::create(project_name.clone() + "/bib/refs.bib")
                .expect("Location must be writable.");
            refs.write_all(BIBTEX_TEMPLATE.as_bytes())
                .expect("File must be writable.");
            println!("Created {} directory.", "bib".blue().bold());
        }
        false => {
            let msg = String::from("No bibliography path needed.");
            println!("{}", msg);
        }
    }

    /* Next create the tex directory regardless, then call `git init` and `git add .` */
    fs::create_dir(project_name.clone() + "/tex").expect("Location must be writable.");

    /* Write the template as bytes to the tex dir, depending on doctype. */
    match config.get_doctype() {
        DocumentType::Article => {
            let mut template =
                File::create(project_name.clone() + "/tex/" + &project_name + ".tex")
                    .expect("Location must be writable.");
            template
                .write_all(ARTICLE_TEMPLATE.as_bytes())
                .expect("File must be writable.");
        }
        DocumentType::Letter => {
            let mut template =
                File::create(project_name.clone() + "/tex/" + &project_name + ".tex")
                    .expect("Location must be writable.");
            template
                .write_all(LETTER_TEMPLATE.as_bytes())
                .expect("File must be writable.");
        }
        DocumentType::Book => {
            todo!();
        }
        _ => {}
    }

    config::write_project_config(&config).expect("Location must be writable.");

    /* These should be about the last things to run. Also write the .gitignore file before this. */
    initialize_git_repository(config.clone()).expect("Git must be installed.");
    add_to_git_repository(config.clone()).expect("Git must be installed.");

    Ok(())
}

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

pub fn create_article(config: ProjectConfig) -> Result<(), io::Error> {
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
                let mut refs = match File::create(name.clone() + "/bib/refs.bib") {
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

    // Because this is an article, we can go ahead and write an article template w/o testing
    // that property. If this method were called outside of creating an article, it would be
    // a bug.
    // TODO: Write out article template...

    Ok(())
}
