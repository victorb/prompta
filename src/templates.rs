use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self};
use std::path::Path;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PlaceholderEntry {
    pub path: String,
    pub language: Option<String>,
    pub directory: Option<String>,
    pub extension: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TemplateMeta {
    pub placeholders: HashMap<String, Vec<PlaceholderEntry>>,
}

pub fn load_meta(path: &Path) -> anyhow::Result<TemplateMeta> {
    let data = fs::read_to_string(path)?;
    let meta: TemplateMeta = serde_json::from_str(&data)?;
    Ok(meta)
}

pub fn save_meta(path: &Path, meta: &TemplateMeta) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(meta)?;
    fs::write(path, json)?;
    Ok(())
}
