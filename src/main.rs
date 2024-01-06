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

use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use serde_derive::{Deserialize, Serialize};

/* MODULES */
pub mod build;
pub mod cli;
pub mod config;
pub mod new;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "texrs")]
#[command(about = "Manage a LaTeX project structure.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Create a new LaTeX project.
    #[command(arg_required_else_help = true)]
    New {
        #[arg(
            long,
            require_equals = true,
            value_name = "WHEN",
            num_args = 0..=1,
            default_value_t = DocumentType::Article,
            default_missing_value = "always",
            value_enum
        )]
        /// Type of template.
        template: DocumentType,
        /// Project name.
        name: String,
    },
    /// Build an existing project.
    #[command(arg_required_else_help = true)]
    Build {
        /// Path to the configuration file.
        path: PathBuf,
    },
    /// Interactive project setup. Recommended.
    #[command(arg_required_else_help = true)]
    Interactive {
        /// Project name.
        name: String,
    },
}

#[derive(ValueEnum, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DocumentType {
    Article,
    Book,
    Thesis,
    Presentation,
    MathArticle,
    Notes,
    Letter,
}

fn main() {
    let args = Cli::parse();

    let mut config = config::ProjectConfig::new();

    match args.command {
        Commands::New { template, name } => match template {
            DocumentType::Article => {
                config.set_name(&name);
                config.set_driver("pdflatex");
                config.set_citations(true);
                config.set_graphics(true);
                config.set_doctype(DocumentType::Article);
                match new::create_directories(config) {
                    Ok(_) => {}
                    Err(err) => eprintln!("{}", err),
                }
            }
            DocumentType::Book => {
                config.set_name(&name);
                config.set_driver("xelatex");
                config.set_citations(true);
                config.set_graphics(true);
                config.set_doctype(DocumentType::Book);
                match new::create_directories(config) {
                    Ok(_) => {}
                    Err(err) => eprintln!("{}", err),
                }
            }
            DocumentType::Thesis => {
                config.set_name(&name);
                config.set_driver("xelatex");
                config.set_citations(true);
                config.set_graphics(true);
                config.set_doctype(DocumentType::Thesis);
                match new::create_directories(config) {
                    Ok(_) => {}
                    Err(err) => eprintln!("{}", err),
                }
            }
            DocumentType::Presentation => {
                config.set_name(&name);
                config.set_driver("xelatex");
                config.set_citations(true);
                config.set_graphics(true);
                config.set_doctype(DocumentType::Presentation);
                match new::create_directories(config) {
                    Ok(_) => {}
                    Err(err) => eprintln!("{}", err),
                }
            }
            DocumentType::MathArticle => {
                config.set_name(&name);
                config.set_driver("xelatex");
                config.set_citations(true);
                config.set_graphics(true);
                config.set_doctype(DocumentType::MathArticle);
                match new::create_directories(config) {
                    Ok(_) => {}
                    Err(err) => eprintln!("{}", err),
                }
            }
            DocumentType::Notes => {
                config.set_name(&name);
                config.set_driver("pdflatex");
                config.set_citations(false);
                config.set_graphics(false);
                config.set_doctype(DocumentType::Notes);
                match new::create_directories(config) {
                    Ok(_) => {}
                    Err(err) => eprintln!("{}", err),
                }
            }
            DocumentType::Letter => {
                config.set_name(&name);
                config.set_driver("xelatex");
                config.set_citations(false);
                config.set_graphics(true);
                config.set_doctype(DocumentType::Letter);
                match new::create_directories(config) {
                    Ok(_) => {}
                    Err(err) => eprintln!("{}", err),
                }
            }
        },
        Commands::Build { path } => {
            let config = build::read_config(path).unwrap();
            match build::build_project(config) {
                Ok(_) => println!("Success!"),
                Err(err) => eprintln!("{}", err),
            }
        }
        Commands::Interactive { name } => {
            let config = cli::config_menu(&name);
            match new::create_directories(config) {
                Ok(_) => {}
                Err(err) => eprintln!("{}", err),
            }
        }
    }
}
