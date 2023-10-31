use crate::{config::*, new};
use clap::ArgMatches;
use colored::*;
use std::process::Command;
use std::{fs, fs::File};
use std::{io, io::Write};

fn build_from_config(config: ProjectConfig) -> Result<(), io::Error> {
    Ok(())
}
