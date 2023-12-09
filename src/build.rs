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
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::process::Command;
use toml;

pub fn read_config(path: &str) -> Result<ProjectConfig, Box<dyn Error>> {
    let path = path.trim();
    let mut toml_str = String::new();
    if path == "." {
        let mut file = File::open("config.toml")?;
        file.read_to_string(&mut toml_str)?;
    } else {
        let mut file = File::open(path.to_owned() + "/config.toml")?;
        file.read_to_string(&mut toml_str)?;
    }
    let config: ProjectConfig = toml::from_str(&toml_str)?;
    Ok(config)
}

pub fn build_project(config: ProjectConfig) -> Result<(), Box<dyn Error>> {
    if let Err(_) = fs::metadata(config.get_name() + "/target") {
        fs::create_dir(config.get_name() + "/target")?;
        println!("[  {}  ] Creating target dir.", "OK".green());
    } else {
        println!("[  {}  ] Target dir. exists; skipping!", "OK".green());
    }

    let mut num_of_passes = 1;
    if config.get_citations() {
        num_of_passes += 1;
    }
    if config.get_graphics() {
        num_of_passes += 1;
    }

    for i in 0..num_of_passes {
        let mut tex_builder = Command::new(config.get_driver());
        tex_builder
            .args(&["../tex/".to_owned() + &config.get_name() + ".tex"])
            .current_dir(config.get_name() + "/target");

        let output = tex_builder.output()?;
        if output.status.success() {
            println!(
                "[  {}  ] Ran {} on pass {}.",
                "OK".green(),
                config.get_driver().as_str().blue(),
                i.to_string().as_str().blue()
            );
            if i == num_of_passes {
                println!("[  {}  ] Document compiled successfully!", "OK".green());
            }
        }

        if i == 1 && config.get_citations() {
            let mut biber = Command::new("biber");
            biber
                .args(&[config.get_name()])
                .current_dir(config.get_name() + "/tex");
            let biber_output = biber.output()?;
            if biber_output.status.success() {
                println!(
                    "[  {}  ] Ran biber on pass {}.",
                    "OK".green(),
                    i.to_string().as_str().blue()
                );
            } else {
                println!(
                    "[ {} ] Biber failed with the following error:",
                    "FAIL".red()
                );
                eprintln!("{}", String::from_utf8_lossy(&biber_output.stderr));
            }
        }
    }

    Ok(())
}
