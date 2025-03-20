use crate::config::*;
use crate::templates::*;
use std::fs::{self, File};

pub fn create_template(name: &str) -> anyhow::Result<()> {
    let md_path = template_md_path(name);
    if md_path.exists() {
        println!("Template {name} already exists at {:?}", md_path);
    } else {
        File::create(&md_path)?;
        println!("Created {:?}", md_path);
    }

    let meta_path = template_meta_path(name);
    if !meta_path.exists() {
        let meta = TemplateMeta::default();
        let json = serde_json::to_string_pretty(&meta)?;
        fs::write(&meta_path, json)?;
    }

    Ok(())
}
