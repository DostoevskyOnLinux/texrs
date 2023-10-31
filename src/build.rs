use crate::config::*;
use clap::ArgMatches;
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
        "Running {} to compile {} in {}/target...",
        config.get_driver().blue(),
        project_file_name.blue(),
        project_name.to_owned()
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
    Ok(())
}

pub fn read_config(args: ArgMatches) -> Result<ProjectConfig, io::Error> {
    let project_path =
        args.get_one::<String>("PATH").expect("required").to_owned() + "/config.toml";
    let mut file = File::open(project_path).expect("config.toml must be present"); // Replace with the path to your TOML file
    let mut toml_str = String::new();
    file.read_to_string(&mut toml_str)?;
    let project_config: ProjectConfig = toml::from_str(&toml_str).expect("TOML syntax invalid.");
    Ok(project_config)
}
