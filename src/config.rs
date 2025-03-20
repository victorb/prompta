use std::env;
use std::fs::{self};
use std::path::{Path, PathBuf};

use dirs::home_dir;

pub fn config_dir() -> PathBuf {
    let mut base = home_dir().unwrap();
    base.push(".config");
    base.push("prompta");
    fs::create_dir_all(&base).expect("Failed to create config dir");
    base
}

pub fn config_dir_str() -> String {
    let dir = config_dir();
    dir.to_string_lossy().into_owned()
}

pub fn resolve_absolute_path(path: &str) -> anyhow::Result<String> {
    // Handle stdin marker
    if path == "-" {
        return Ok(path.to_string());
    }

    let path_buf = if Path::new(path).is_absolute() {
        PathBuf::from(path)
    } else {
        env::current_dir()?.join(path)
    };

    Ok(path_buf.canonicalize()?.to_string_lossy().into_owned())
}

pub fn template_md_path(name: &str) -> PathBuf {
    config_dir().join(format!("{name}.md"))
}

pub fn template_meta_path(name: &str) -> PathBuf {
    config_dir().join(format!("{name}.json"))
}
