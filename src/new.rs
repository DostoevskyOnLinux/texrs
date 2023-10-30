use std::{fs, fs::File};
use std::io;
use std::io::prelude::*;
use std::process::Command;
use colored::*;
use crate::config::*;

const ARTICLE_TEMPLATE: &str = include_str!("article.tex");

pub fn create_dir_structure(config: ProjectConfig) -> Result<(), io::Error> {
    // Always create a new directory.
    let project_name = &config.get_name().to_owned();
    fs::create_dir(project_name.clone()).expect("Location must be writable.");

    match config.get_graphics() {
        // If true, create graphics dir.
        true => {
            fs::create_dir(project_name.clone() + "/graphics")
                .expect("Location must be writable.");
            let msg = String::from("graphics");
            println!("Created {} directory.", msg.blue().bold());
        },
        false => {
            let msg = String::from("No graphics path needed.");
            println!("{}", msg);
        }
    }

    match config.get_citations() {
        // If true, create bib dir.
        // TODO: This should also write a .bib file to the new directory.
        true => {
            fs::create_dir(project_name.clone() + "/bib")
                .expect("Location must be writable.");
            let msg = String::from("bib");
            println!("Created {} directory.", msg.blue().bold());
        },
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
            let template = File::create(project_name.clone() + "/tex" + &project_name + ".tex").expect("Location must be writable.");
            template.write_all(ARTICLE_TEMPLATE.as_bytes()).expect("File must be writable.");
        },
        _ => unimplemented!() // TODO: Implement cases for Book & Letter.
    }

    /* These should be about the last things to run. Also write the .gitignore file before this. */
    initialize_git_repository(config.clone());
    add_to_git_repository(config.clone());

    Ok(())
}

fn initialize_git_repository(config: ProjectConfig) -> Result<(), io::Error> {
    let project_directory = config.get_name();

    let mut git_init = Command::new("git").args(&["init"]).current_dir(project_directory);
    let output = git_init.output().expect("Git failed to initialize.");

    if output.status.success() {
        println!("Git repository initialized successfully in {}.", project_directory.blue());
    } else {
        eprintln!("Failed to initialize Git repository:\n{}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}

fn add_to_git_repository(config: ProjectConfig) -> Result<(), io::Error> {
    let project_directory = config.get_name();

    let mut git_add = Command::new("git").args(&["add", "."]).current_dir(project_directory);
    let output = git_add.output().expect("Git failed to add files.");

    if output.status.success() {
        println!("Git repository populated successfully in {}.", project_directory.blue());
    } else {
        eprintln!("Failed to populate Git repository:\n{}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}
