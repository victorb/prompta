use crate::config::*;
use crate::templates::*;
use crate::cli::commands::*;

pub fn add_to_var(
    name: &str,
    placeholder_name: &str,
    file: &str,
    language: Option<String>,
    directory: Option<String>,
    extension: Option<String>,
) -> anyhow::Result<()> {
    check_dir_and_ext_args_together(&directory, &extension);

    let meta_path = template_meta_path(name);
    if !meta_path.exists() {
        println!("Template {name} metadata does not exist. Create it first.");
        return Ok(());
    }

    let mut meta = load_meta(&meta_path)?;
    let absolute_path = resolve_absolute_path(file)?;

    let entries = meta
        .placeholders
        .entry(placeholder_name.to_string())
        .or_default();
    entries.push(PlaceholderEntry {
        path: absolute_path,
        language,
        directory,
        extension,
    });

    save_meta(&meta_path, &meta)?;
    println!("Added new entry to placeholder '{placeholder_name}' in '{name}'");
    Ok(())
}
