use regex::Regex;

/// Looks for occurrences of `(SHELL some command here)` and replaces them
/// with the combined stdout+stderr output of running that command in a shell.
pub fn handle_shell(original_text: &str) -> anyhow::Result<String> {
    let re = Regex::new(r"\(SHELL\s+([^)]+)\)")?;
    let mut result = original_text.to_string();

    loop {
        let mut replaced_any = false;
        let text_snapshot = result.clone();

        for captures in re.captures_iter(&text_snapshot) {
            let command_str = &captures[1];

            // Run the command via `sh -c`
            let output = std::process::Command::new("sh")
                .arg("-c")
                .arg(command_str)
                .output()
                .map_err(|e| anyhow::anyhow!("Failed to execute shell command: {}", e))?;

            // If the command returned non-zero, bail out with an error
            if !output.status.success() {
                anyhow::bail!(
                    "Shell command exited with error (status {:?}): {}",
                    output.status.code(),
                    command_str
                );
            }

            // Combine stdout and stderr into one String
            let combined_output = format!(
                "{}{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );

            // Replace only the first occurrence in this loop iteration
            result = result.replacen(&captures[0], &combined_output, 1);
            replaced_any = true;
        }

        if !replaced_any {
            break;
        }
    }

    Ok(result)
}

// Test a single shell command that should succeed
#[test]
fn test_handle_shell_single_command() -> anyhow::Result<()> {
    let input = "Some text (SHELL echo HelloWorld) more text";
    let output = handle_shell(input)?;
    let expected = "Some text HelloWorld\n more text";
    assert_eq!(output, expected);
    Ok(())
}

#[test]
fn test_handle_shell_with_bc() -> anyhow::Result<()> {
    let input = "Some wonder what two plus two is. Wonder no more: (SHELL echo '2 + 2' | bc)";
    let output = handle_shell(input)?;
    let expected = "Some wonder what two plus two is. Wonder no more: 4\n";
    assert_eq!(output, expected);
    Ok(())
}


// Test multiple commands in the same string
#[test]
fn test_handle_shell_multiple_commands() -> anyhow::Result<()> {
    let input = "(SHELL echo First) and (SHELL echo Second)";
    let output = handle_shell(input)?;
    let expected = "First\n and Second\n";
    assert_eq!(output, expected);
    Ok(())
}

// Test that a failing command triggers an error
#[test]
fn test_handle_shell_failure() {
    // 'false' is a shell built-in that exits with non-zero code
    let input = "Will fail: (SHELL false)";
    let result = handle_shell(input);
    assert!(result.is_err(), "Expected an error but got OK");
}
