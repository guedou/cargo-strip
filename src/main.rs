// Copyright (C) 2019 Guillaume Valadon <guillaume@valadon.net>

use cargo_metadata::MetadataCommand;
use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn strip_binary(filepath: &mut PathBuf) -> Result<(), String> {
    // Retrieve files metadata
    filepath.set_extension("cargo-strip_info");
    let strip_info_metadata = fs::metadata(filepath.as_path());
    filepath.set_extension("");

    let binary_metadata = fs::metadata(filepath.as_path());

    // Determine if the binary needs to be stripped
    let strip_needed = match (binary_metadata, strip_info_metadata) {
        (Ok(_), Err(s)) if s.kind() == ErrorKind::NotFound => true,
        (Err(_), Err(_)) => false,
        (Ok(b), Ok(s)) => {
            let s_modified = s
                .modified()
                .or_else(|_| Err("Modification time unavailable!"))?;
            let b_modified = b
                .modified()
                .or_else(|_| Err("Modification time unavailable!"))?;
            s_modified <= b_modified
        }
        (_, _) => false,
    };

    if !strip_needed {
        return Ok(());
    }

    // Strip the binary
    Command::new("strip")
        .arg(&filepath)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .or_else(|_| Err("Cannot execute strip!"))?;
    println!("{:?} stripped!", filepath);

    // Create the .cargo-strip_info file
    filepath.set_extension("cargo-strip_info");
    File::create(&filepath)
        .or_else(|_| Err("Cannot create the .cargo-strip_info file!"))?;
    filepath.set_extension("");

    Ok(())
}

fn main() -> Result<(), String> {
    // Check if the strip binary is available
    Command::new("strip")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .or_else(|_| Err("Please install strip!"))?;

    // Retrieve package information
    let metadata = MetadataCommand::new()
        .manifest_path("./Cargo.toml")
        .no_deps()
        .exec()
        .or_else(|_| Err("Cannot parse Cargo.toml!"))?;

    // Iterate over the target directory
    for entry in fs::read_dir(metadata.target_directory)
        .or_else(|_| Err("Cannot access the target directory!"))?
    {
        // Identify directories and continue otherwise
        let entry = entry.or_else(|_| Err("IO error!"))?;
        if !entry
            .file_type()
            .or_else(|_| Err("Cannot get file type!"))?
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

            strip_binary(&mut path)?;
        }
    }

    Ok(())
}
