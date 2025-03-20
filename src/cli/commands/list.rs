use crate::config::*;
use std::fs::{self};

pub fn list_templates() -> anyhow::Result<()> {
    let mut matches = Vec::new();
    for entry in fs::read_dir(config_dir())? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map_or(false, |ext| ext == "md") {
            if let Some(file_name) = path.file_stem() {
                matches.push(file_name.to_string_lossy().into_owned());
            }
        }
    }

    matches.sort();

    println!("Available templates:\n");
    for m in matches {
        println!("\t{m}");
    }

    Ok(())
}
