// Copyright (C) 2019 Guillaume Valadon <guillaume@valadon.net>

use cargo_metadata::MetadataCommand;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn strip_binary(filepath: &PathBuf) -> Result<(), String> {
    // TODO: .cargo-strip_info

    // Strip the binary
    Command::new("strip")
        .arg(filepath)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .or_else(|_e| Err("Cannot execute strip!"))?;

    Ok(())
}

fn main() -> Result<(), String> {
    // Check if the strip binary is available
    Command::new("strip")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .or_else(|_e| Err("Please install strip!"))?;

    // Retrieve package information
    let metadata = MetadataCommand::new()
        .manifest_path("./Cargo.toml")
        .no_deps()
        .exec()
        .or_else(|_e| Err("Cannot parse Cargo.toml!"))?;

    // Iterate over the target directory
    for entry in fs::read_dir(metadata.target_directory)
        .or_else(|_e| Err("Cannot access the target directory!"))?
    {
        // Identify directories and continue otherwise
        let entry = entry.or_else(|_e| Err("IO error!"))?;
        if !entry
            .file_type()
            .or_else(|_e| Err("Cannot get file type!"))?
            .is_dir()
        {
            continue;
        }

        // Iterate over possible binaries
        for binary in metadata
            .packages
            .iter()
            .flat_map(|p| &p.targets)
            .filter(|t| t.kind == vec!["bin"])
            .filter(|t| t.kind == vec!["bin"])
            .map(|x| &x.name)
        {
            // Check if the binary exists in the current directory
            let mut path = entry.path();
            path.push(binary);
            if !path.is_file() {
                continue;
            }

            strip_binary(&path)?;
            println!("{:?} stripped!", path);
        }
    }

    Ok(())
}
