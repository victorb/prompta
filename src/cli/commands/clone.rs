use crate::config::*;
use crate::templates::*;
use std::fs;

pub fn clone_template(source: &str, destination: &str) -> anyhow::Result<()> {
    // Get paths for source template
    let source_md_path = template_md_path(source);
    let source_meta_path = template_meta_path(source);

    // Get paths for destination template
    let dest_md_path = template_md_path(destination);
    let dest_meta_path = template_meta_path(destination);

    // Validate that source exists
    if !source_md_path.exists() || !source_meta_path.exists() {
        anyhow::bail!(
            "Source template '{source}' does not exist or is incomplete. Both MD and JSON files must exist."
        );
    }

    // Validate that destination doesn't exist
    if dest_md_path.exists() || dest_meta_path.exists() {
        anyhow::bail!(
            "Destination template '{destination}' already exists. Use a different name or remove it first."
        );
    }

    // Copy the markdown template
    fs::copy(&source_md_path, &dest_md_path)?;

    // Load and save the metadata
    let meta = load_meta(&source_meta_path)?;
    save_meta(&dest_meta_path, &meta)?;

    println!("Successfully cloned template '{source}' to '{destination}'");

    Ok(())
}
