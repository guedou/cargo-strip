// Copyright (C) 2020 Guillaume Valadon <guillaume@valadon.net>

use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use cargo_metadata::MetadataCommand;
use clap::{crate_version, App, Arg, SubCommand};

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

    let filesize_before = fs::metadata(&filepath)
        .or_else(|_| Err("Cannot get file size!"))?
        .len();

    // Strip the binary
    Command::new("strip")
        .arg(&filepath)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .or_else(|_| Err("Cannot execute strip!"))?;

    let filesize_after = fs::metadata(&filepath)
        .or_else(|_| Err("Cannot get file size!"))?
        .len();

    println!(
        "{:?} stripped (reduced by {} kB)!",
        filepath,
        (filesize_before - filesize_after) / 1024
    );

    // Create the .cargo-strip_info file
    filepath.set_extension("cargo-strip_info");
    File::create(&filepath).or_else(|_| Err("Cannot create the .cargo-strip_info file!"))?;
    filepath.set_extension("");

    Ok(())
}

fn main() -> Result<(), String> {
    // Parse command line arguments
    let matches = App::new("cargo-strip - reduces the size of binaries using the `strip` command")
        .version(&crate_version!()[..])
        // cargo subcommand trick from https://github.com/clap-rs/clap/issues/937
        .bin_name("cargo")
        .subcommand(
            SubCommand::with_name("strip").arg(
                Arg::with_name("target")
                    .short("t")
                    .long("target")
                    .takes_value(true)
                    .help("name of the target to strip"),
            ),
        )
        .get_matches();

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
            .map(|x| &x.name)
        {
            // Build a path containing the specified target
            let mut path = if matches.is_present("target") {
                let path = entry.path();
                // Remove the last element from the path
                let root_path = path
                    .ancestors()
                    .nth(1)
                    .ok_or("Cannot remove build from path!")?;

                // Add the target & the last element of the path
                root_path.join(matches.value_of("target").unwrap()).join(
                    path.components()
                        .last()
                        .ok_or("Cannot extract build from path!")?
                        .as_os_str(),
                )
            } else {
                entry.path()
            };

            // Check if the binary exists in the current directory
            path.push(binary);
            if !path.is_file() {
                continue;
            }

            strip_binary(&mut path)?;
        }
    }

    Ok(())
}
