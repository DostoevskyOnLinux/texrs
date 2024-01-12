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
use std::error::Error;
use std::io;
use std::io::Write;
use std::process::Command;
use std::{fs, fs::File};

const ARTICLE_TEMPLATE: &str = include_str!("../res/article.tex");
const LETTER_TEMPLATE: &str = include_str!("../res/letter.tex");
const RECIPE_TEMPLATE: &str = include_str!("../res/recipe.tex");
const BIBTEX_TEMPLATE: &str = include_str!("../res/refs.bib");
const NOTES_TEMPLATE: &str = include_str!("../res/notes.tex");
const BOOK_TEMPLATE: &str = include_str!("../res/book.tex");

/* -------------------------------------------------------------------- */
/// This method creates a directory structure based on a ProjectConfig
/// struct passed in by the caller. It returns a result depending on the
/// file IO, which may experience an issue.
///
/// ## Usage
///
/// ```rust
/// create_directories(config).expect("File IO failed.");
/// ```
/* -------------------------------------------------------------------- */
pub fn create_directories(config: ProjectConfig) -> Result<(), Box<dyn Error>> {
    fs::create_dir(config.get_name())?;
    if config.get_graphics() {
        fs::create_dir(config.get_name() + "/graphics")?;
        println!("[  {}  ] Created graphics dir.", "OK".green());
    } else {
        println!("[ {} ] Skipped graphics dir.", "WARN".yellow());
    }

    if config.get_citations() {
        fs::create_dir(config.get_name() + "/bib")?;
        let mut file = File::create(config.get_name() + "/bib/refs.bib")?;
        file.write_all(BIBTEX_TEMPLATE.as_bytes())?;
        println!("[  {}  ] Created references dir.", "OK".green());
    } else {
        println!("[ {} ] Skipped references dir.", "WARN".yellow());
    }

    fs::create_dir(config.get_name() + "/tex")?;

    match config.get_doctype() {
        DocumentType::Article => {
            let mut file =
                File::create(config.get_name() + "/tex/" + config.get_name().as_str() + ".tex")?;
            file.write_all(ARTICLE_TEMPLATE.as_bytes())?;
            println!("[  {}  ] Created tex dir.", "OK".green());
        }
        DocumentType::Book => {
            let mut file =
                File::create(config.get_name() + "/tex/" + config.get_name().as_str() + ".tex")?;
            file.write_all(BOOK_TEMPLATE.as_bytes())?;
            println!("[  {}  ] Created tex dir.", "OK".green());
        }
        DocumentType::Thesis => {
            let mut file =
                File::create(config.get_name() + "/tex/" + config.get_name().as_str() + ".tex")?;
            file.write_all(ARTICLE_TEMPLATE.as_bytes())?;
            println!("[  {}  ] Created tex dir.", "OK".green());
        }
        DocumentType::MathArticle => {
            let mut file =
                File::create(config.get_name() + "/tex/" + config.get_name().as_str() + ".tex")?;
            file.write_all(ARTICLE_TEMPLATE.as_bytes())?;
            println!("[  {}  ] Created tex dir.", "OK".green());
        }
        DocumentType::Presentation => {
            let mut file =
                File::create(config.get_name() + "/tex/" + config.get_name().as_str() + ".tex")?;
            file.write_all(ARTICLE_TEMPLATE.as_bytes())?;
            println!("[  {}  ] Created tex dir.", "OK".green());
        }
        DocumentType::Notes => {
            let mut file =
                File::create(config.get_name() + "/tex/" + config.get_name().as_str() + ".tex")?;
            file.write_all(NOTES_TEMPLATE.as_bytes())?;
            println!("[  {}  ] Created tex dir.", "OK".green());
        }
        DocumentType::Letter => {
            let mut file =
                File::create(config.get_name() + "/tex/" + config.get_name().as_str() + ".tex")?;
            file.write_all(LETTER_TEMPLATE.as_bytes())?;
            println!("[  {}  ] Created tex dir.", "OK".green());
        }
        DocumentType::Recipe => {
            let mut file =
                File::create(config.get_name() + "/tex/" + config.get_name().as_str() + ".tex")?;
            file.write_all(RECIPE_TEMPLATE.as_bytes())?;
            println!("[  {}  ] Created tex dir.", "OK".green());
        }
    }

    match write_project_config(&config) {
        Ok(_) => println!("[  {}  ] Project config written.", "OK".green()),
        Err(err) => {
            println!("[ {} ]", "FAIL".red());
            eprintln!("{}", err);
        }
    }

    match git_init(config.clone()) {
        Ok(_) => {}
        Err(err) => {
            println!("[ {} ] Git init failed.", "FAIL".red());
            eprintln!("{}", err);
        }
    }
    match git_add(config.clone()) {
        Ok(_) => {}
        Err(err) => {
            println!("[ {} ] Git add failed.", "FAIL".red());
            eprintln!("{}", err);
        }
    }
    match git_commit(config) {
        Ok(_) => {}
        Err(err) => {
            println!("[ {} ] Git commit failed.", "FAIL".red());
            eprintln!("{}", err);
        }
    }

    Ok(())
}

fn git_init(config: ProjectConfig) -> Result<(), Box<dyn Error>> {
    let mut git_init = Command::new("git");
    let output = git_init
        .args(["init"])
        .current_dir(config.get_name())
        .output()?;
    if output.status.success() {
        println!("[  {}  ] Git repository initialized.", "OK".green());
        Ok(())
    } else {
        Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            "git failed.",
        )))
    }
}

fn git_add(config: ProjectConfig) -> Result<(), Box<dyn Error>> {
    let mut git_add = Command::new("git");
    if git_add
        .args(["add", "."])
        .current_dir(config.get_name())
        .output()?
        .status
        .success()
    {
        println!("[  {}  ] Git added files.", "OK".green());
        Ok(())
    } else {
        Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            "git failed.",
        )))
    }
}

fn git_commit(config: ProjectConfig) -> Result<(), Box<dyn Error>> {
    let mut git_commit = Command::new("git");
    if git_commit
        .args(["commit", "-m", "\"Initialize repository.\""])
        .current_dir(config.get_name())
        .output()?
        .status
        .success()
    {
        println!("[  {}  ] Git repository committed.", "OK".green());
        Ok(())
    } else {
        Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            "git failed.",
        )))
    }
}
