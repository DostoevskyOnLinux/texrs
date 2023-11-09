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
// | <https://github.com/ethanbarry> is the author's profile.                                                                          |
// + - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - +

use crate::config::*;

use colored::*;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::process::Command;
use toml;

pub fn build_from_config(config: ProjectConfig) -> Result<(), io::Error> {
    let project_name = &config.get_name().to_owned();
    let target_name = project_name.clone() + "/target";
    // Test whether the target dir. exists already.
    if let Err(_) = fs::metadata(&target_name) {
        match fs::create_dir(&target_name) {
            Ok(_) => println!("Created {} directory.", target_name.clone().blue()),
            Err(err) => eprintln!(
                "Failed to create {} directory. Error: {}",
                target_name.clone().blue(),
                err
            ),
        }
    } else {
        println!(
            "Directory {} exists; skipping...",
            target_name.clone().blue()
        );
    }

    let mut tex_builder = Command::new(config.get_driver());
    tex_builder
        .args(&["../tex/".to_owned() + project_name + ".tex"])
        .current_dir(project_name.clone() + "/target");
    let project_file_name = "../tex/".to_owned() + project_name + ".tex";
    println!(
        "Running {} to compile {} in {}{}...",
        config.get_driver().blue(),
        project_file_name.blue(),
        project_name.to_owned().blue(),
        "/target".blue()
    );

    let output = tex_builder.output().expect("Driver failed to build.");
    if output.status.success() {
        println!("LaTeX compiled successfully in {}.", target_name.blue());
    } else {
        eprintln!(
            "Failed to compile LaTeX:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    if config.get_citations() {
        let mut biber = Command::new("biber");
        biber
            .args(&[project_name])
            .current_dir(project_name.to_owned() + "/tex");
        println!("Running {} to compile citations...", "biber".blue());

        let biber_output = biber.output().expect("Biber failed to initialize.");
        if biber_output.status.success() {
            println!("Citations generated for {}...", project_name.green());
        } else {
            eprintln!(
                "Failed to generate citations for {}:\n{}",
                project_name.red(),
                String::from_utf8_lossy(&biber_output.stderr)
            );
        }
    }

    if config.get_citations() || config.get_graphics() {
        let mut final_output = Command::new(config.get_driver());
        final_output
            .args(&["../tex/".to_owned() + project_name + ".tex"])
            .current_dir(project_name.clone() + "/target");
        let project_file_name = "../tex/".to_owned() + project_name + ".tex";
        println!(
            "Running {} to compile {} in {}{}...",
            config.get_driver().blue(),
            project_file_name.blue(),
            project_name.to_owned().blue(),
            "/target".blue()
        );

        let output = tex_builder.output().expect("Driver failed to build.");
        if output.status.success() {
            println!("LaTeX compiled successfully in {}.", target_name.blue());
        } else {
            eprintln!(
                "Failed to compile LaTeX:\n{}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }
    Ok(())
}

pub fn read_config(args: &str) -> Result<ProjectConfig, io::Error> {
    let project_path = args.to_owned() + "/config.toml";
    let mut file = File::open(project_path).expect("config.toml must be present"); // Replace with the path to your TOML file
    let mut toml_str = String::new();
    file.read_to_string(&mut toml_str)?;
    let project_config: ProjectConfig = toml::from_str(&toml_str).expect("TOML syntax invalid.");
    Ok(project_config)
}
