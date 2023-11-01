use crate::config::{self, *};
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
        _ => unimplemented!(), // TODO: Implement cases for Book. We need a good book template...
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

    Ok(())
}
