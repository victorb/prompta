pub fn handle_comment(original_text: &str) -> anyhow::Result<String> {
    let mut new_doc: Vec<String> = vec![];
    for line in original_text.split("\n") {
        if line.trim().starts_with("--") {
            continue;
        }
        new_doc.push(line.to_string());
    }
    Ok(new_doc.join("\n"))
}

#[test]
fn test_handle_comment() {
    let input = r#"
This is a line
This is another line
-- Here is a comment
And here is one not
    "#;
    let result = handle_comment(input).expect("Something went wrong");
    assert_ne!(input, result);
}

#[test]
fn test_handle_comment_whitespace_prefix() {
    let input = r#"
            This is a line
            This is another line
            -- Here is a comment
            And here is one not
    "#;
    let result = handle_comment(input).expect("Something went wrong");
    assert_ne!(input, result);
}
