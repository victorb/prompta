use crate::config::*;
use regex::Regex;
use std::{fs::{self}, path::Path};

/// Looks for occurrences of `(INCLUDE path/to/file.md)` and replaces them
/// with the contents of `~/.config/prompta/path/to/file.md`.
pub fn handle_include_with_reader<F, G>(
    original_text: &str,
    config_directory: &Path,
    read_file: F,
    file_exists: G,
) -> anyhow::Result<String>
where
    F: Fn(&Path) -> std::io::Result<String>,
    G: Fn(&Path) -> bool,
{
    let re = Regex::new(r"\(INCLUDE\s+([^)]+)\)")?;
    let mut result = original_text.to_string();

    loop {
        let mut replaced_any = false;
        let text_snapshot = result.clone();

        for captures in re.captures_iter(&text_snapshot) {
            let include_path = &captures[1];
            let full_path = config_directory.join(include_path);

            if !file_exists(&full_path) {
                anyhow::bail!("Include file not found: {:?}", full_path);
            }
            let content = read_file(&full_path)?;
            result = result.replacen(&captures[0], &content, 1);
            replaced_any = true;
        }

        if !replaced_any {
            break;
        }
    }

    Ok(result)
}

pub fn handle_include(original_text: &str) -> anyhow::Result<String> {
    handle_include_with_reader(
        original_text,
        &config_dir(),
        |path| fs::read_to_string(path),
        |path| path.exists(),
    )
}


#[test]
fn test_handle_include() -> anyhow::Result<()> {
    use std::{collections::HashMap, path::Path};

    let mut mocks = HashMap::new();
    mocks.insert("foo/bar.md", "Hello from bar".to_string());
    mocks.insert("baz/qux.md", "Hello from qux".to_string());

    let fake_exists = |p: &Path| mocks.contains_key(p.to_str().unwrap());
    let fake_reader = |p: &Path| {
        mocks
            .get(p.to_str().unwrap())
            .cloned()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Mock file not found"))
    };

    let input = "(INCLUDE foo/bar.md)\n(INCLUDE baz/qux.md)";

    // Now we can pass in a made-up directory path, or just rely on the key matching.
    let output = handle_include_with_reader(input, Path::new(""), fake_reader, fake_exists)?;
    assert_eq!(output, "Hello from bar\nHello from qux");

    Ok(())
}