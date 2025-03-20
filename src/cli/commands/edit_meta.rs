use crate::config::*;
use std::env;
use std::process::Command;

pub fn edit_template_meta(name: &str) -> anyhow::Result<()> {
    let md_path = template_meta_path(name);
    if !md_path.exists() {
        println!("Template {name} does not exist. Create it first.");
        return Ok(());
    }
    let editor = env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
    Command::new(editor)
        .arg(&md_path)
        .status()
        .expect("Failed to open file in editor");
    Ok(())
}
