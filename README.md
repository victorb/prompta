## `prompta`
> LLM Prompt manager for easily managing reusable prompts with dynamic file contents

This is a small CLI meant for terminal users who want to have reusable LLM prompts that are easy to maintain, change and "render", with dynamic content that might change everytime you run it.

Developed because I started using multiple LLMs, tend to restart conversations often (since LLMs lose track of the conversation REALLY quickly).

### Features

- Create and manage template prompts
- Replace variables in templates with dynamic content (files, code snippets, directory contents)
- Include content from other files
- Execute shell commands and include their output
- Simple comment system

## Installation

For now, clone the repository and run `cargo install --path .`

## Quick Start

1. Create a new template:
   ```bash
   prompta create mytemplate
   ```

2. Edit the template:
   ```bash
   prompta edit mytemplate
   ```

3. Add a file to a placeholder variable:
   ```bash
   prompta add-to-var mytemplate CODE --file=src/main.rs --language=rust
   ```

4. Output the rendered template:
   ```bash
   prompta output mytemplate
   ```

## Command Reference

### `prompta create <name>`
Creates a new empty template.

### `prompta edit <name>`
Opens the template in your default editor (from `$EDITOR` environment variable).

### `prompta edit-meta <name>`
Edits the template's metadata JSON file.

### `prompta list`
Shows all available templates.

### `prompta clone <source> <destination>`
Copies an existing template to a new name.

### `prompta add-to-var <name> <placeholder> [options]`
Adds content to a placeholder variable.

Options:
- `--file, -f <path>`: Path to a file (use "-" for stdin)
- `--language, -l <lang>`: Specify code language for syntax highlighting
- `--directory, -d <path>`: Directory to include files from
- `--extension, -s <ext>`: File extension to filter by (required with --directory)

### `prompta output <name> [--var KEY=VALUE...]`
Renders the template with all placeholders replaced.

## Template Features

### Placeholders
Use `$VARIABLE` syntax for placeholders that will be replaced when rendered:
```
Here is my source code:
$CODE
```

### Special Placeholders
- `$DATETIME`: Current date and time
- `$STDIN`: Content from standard input

### Macros
- `(INCLUDE path/to/file.md)`: Includes content from another file
- `(SHELL command)`: Executes a shell command and includes its output

### Comments
Lines starting with `--` are treated as comments and removed when rendering:
```
-- This is a comment
This line will be included
```

## Example Use Cases

1. **Code Review Template**:
   ```markdown
   # Code Review Request
   
   Date: $DATETIME
   
   ## My Code:
   $SOURCE_CODE
   
   ## What I'm trying to do:
   $PROBLEM_DESCRIPTION
   ```

2. **Project Summary**:
   ```markdown
   # Project Overview
   
   Here are all the Rust files in my project:
   $RUST_FILES
   
   Current git status:
   (SHELL git status)
   
   Latest commit:
   (SHELL git log -1)
   ```

3. **Documentation Helper**:
   ```markdown
   I need help documenting this function:
   $FUNCTION
   
   Please write comprehensive documentation explaining:
   - What it does
   - Parameters
   - Return value
   - Any edge cases
   ```

## Tips
- Use the `--var` option with `output` to override variables at runtime: `prompta output mytemplate --var CODE="console.log('hello')"` 
- Create a standard set of templates for common tasks
- Use the `(INCLUDE)` macro to share common content between templates


# License

MIT 2025 - Victor Bjelkholm
