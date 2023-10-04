use anyhow::Result;
use clap::{Parser, Subcommand};

use std::{
    fs::{self, File},
    io::{Read, Write},
    os::unix::prelude::PermissionsExt,
    path::PathBuf,
    process,
};

use fakemall::set;

/// A simple tool for building fake command line interfaces
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Execute a command from a set
    Exec {
        /// Set to execute command from
        set: String,
        /// Command to execute
        command: String,
    },
    /// Build an environment from a set
    Build {
        /// Set to build
        set: String,
        /// Path to build set in
        path: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Exec { set, command } => exec(set, command.trim()),
        Commands::Build { set, path } => build(set, path),
    }
}

fn exec(set: &str, command: &str) -> Result<()> {
    let mut toml_file = File::open(set)?;
    let mut toml_string = String::new();

    toml_file.read_to_string(&mut toml_string)?;
    let set = set::parse_toml(toml_string)?;

    if let Some(c) = set.commands.iter().find(|c| c.matches == command) {
        println!("{}", c.output);
        Ok(())
    } else {
        println!("{}: command not found", command);
        process::exit(127)
    }
}

fn build(set_path: &str, path: &str) -> Result<()> {
    let bin_path = PathBuf::from(path).join("bin");
    fs::create_dir_all(&bin_path)?;

    let mut toml_file = File::open(set_path)?;
    let mut toml_string = String::new();

    toml_file.read_to_string(&mut toml_string)?;
    let set = set::parse_toml(toml_string)?;

    for command in set.commands {
        let file_path = PathBuf::from(&bin_path).join(&command.matches);
        let mut file = File::create(file_path)?;
        let metadata = file.metadata()?;
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o755);
        file.set_permissions(permissions)?;

        let shell_script = format!(
            "#!/usr/bin/env sh\nfakemall exec {} \"{} $*\"",
            set_path, command.matches
        );

        file.write_all(shell_script.as_bytes())?;
    }

    Ok(())
}
