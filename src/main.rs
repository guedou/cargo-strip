// Guillaume Valadon <guillaume@valadon.net>

use cargo_metadata::MetadataCommand;
use std::fs;
use std::process::Command;

fn main() {
    let status = Command::new("strip").status();
    if status.is_err() {
        eprintln!("Please install strip!");
        return;
    }

    let metadata = MetadataCommand::new()
        .manifest_path("./Cargo.toml")
        .no_deps()
        .exec()
        .unwrap(); // TODO: unwrap

    for entry in fs::read_dir(metadata.target_directory).unwrap() { // TODO: unwrap
        let entry = entry.unwrap(); // TODO: unwrap
        if !entry.file_type().unwrap().is_dir() { // TODO: unwrap
            continue;
        }
        for binary in metadata
            .packages
            .iter()
            .flat_map(|p| &p.targets)
            .filter(|t| t.kind == vec!["bin"])
            .filter(|t| t.kind == vec!["bin"])
            .map(|x| &x.name)
        {
            let mut path = entry.path();
            path.push(binary);
            if !path.is_file() {
                continue;
            }
            // TODO: .cargo-strip_info
            let status = Command::new("strip")
                .arg(&path)
                .status();
            let path = path.as_os_str().to_str().unwrap(); // TODO: unwrap
            if status.is_err() {
                eprintln!("Executing strip failed on {} !", path);
                return;
            }
            println!("{} striped!", path);
        }
    }
}
