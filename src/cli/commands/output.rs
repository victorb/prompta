use crate::config::*;
use crate::templates::*;
use chrono::Local;
use std::collections::HashMap;
use std::fs::{self};
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

use crate::macros::comment::handle_comment;
use crate::macros::include::handle_include;
use crate::macros::shell::handle_shell;

/// Print the final template with placeholders replaced.
/// The `variables` parameter holds runtime overrides (KEY=VALUE).
pub fn show_output(name: &str, variables: &[(String, String)]) -> anyhow::Result<()> {
    let md_path = template_md_path(name);
    let meta_path = template_meta_path(name);

    if !md_path.exists() || !meta_path.exists() {
        println!("Template {name} not found or metadata missing.");
        return Ok(());
    }

    let mut template = fs::read_to_string(&md_path)?;
    let meta = load_meta(&meta_path)?;

    template = handle_comment(&template)?;

    template = handle_include(&template)?;

    template = handle_shell(&template)?;

    let mut final_map = HashMap::new();

    for (key, entries) in &meta.placeholders {
        final_map.insert(key.to_string(), combine_entries(entries));
    }

    // Apply runtime overrides last
    for (key, val) in variables {
        final_map.insert(key.clone(), val.clone());
    }

    if template.contains("$DATETIME") {
        let now_str = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        final_map.insert("DATETIME".to_string(), now_str);
    }

    if template.contains("$STDIN") {
        // Read once from stdin
        let stdin_content = read_content("-")?;
        final_map.insert("STDIN".to_string(), stdin_content);
    }

    // Replace all `$KEY` placeholders in the template with their final values.
    let mut output_str = template;
    for (key, final_value) in &final_map {
        let marker = format!("${key}");
        output_str = output_str.replace(&marker, final_value);
    }

    println!("{}", output_str);
    Ok(())
}

fn add_to_results(path: &str, lang: &str, content: &str) -> String {
    format!(
        r#"
File `{}`:
```{}
{}
```
"#,
        path.to_string(),
        lang,
        &content
    )
}

fn gather_files_recursively(dir: &Path, extension: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir).expect("Couldn't read path") {
        let path = entry.expect("Couldn't read subpath").path();
        if path.is_dir() {
            files.extend(gather_files_recursively(&path, extension));
        } else if path
            .extension()
            .map_or(false, |ext| ext.to_str() == Some(extension))
        {
            files.push(path);
        }
    }
    files
}

fn combine_entry(entry: &PlaceholderEntry) -> String {
    let paths = if let Some(dir) = &entry.directory {
        let ext = entry
            .extension
            .as_ref()
            .expect("If `directory` is specified, `extension` is required");
        gather_files_recursively(Path::new(dir), ext)
    } else {
        vec![PathBuf::from(&entry.path)]
    };

    let mut result = String::new();
    for p in paths {
        let content = read_content(p.to_string_lossy().as_ref()).unwrap();
        if let Some(lang) = &entry.language {
            result.push_str(&add_to_results(&p.to_string_lossy(), lang, &content));
        } else {
            result.push_str(&content);
            result.push('\n');
        }
    }
    result
}

fn combine_entries(entries: &[PlaceholderEntry]) -> String {
    entries.iter().map(combine_entry).collect()
}

/// If the user gave a path == "-", read from stdin; else read the file.
fn read_content(path_or_dash: &str) -> anyhow::Result<String> {
    if path_or_dash == "-" {
        use std::io::IsTerminal;

        if std::io::stdin().is_terminal() {
            return Ok("No stdin".to_string());
        }

        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer)?;
        Ok(buffer)
    } else {
        Ok(std::fs::read_to_string(path_or_dash)?)
    }
}
