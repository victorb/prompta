use crate::config::config_dir_str;
use clap::{Parser, Subcommand, ValueHint};

pub mod commands;

use crate::cli::commands::add_to_var::add_to_var;
use crate::cli::commands::clone::clone_template;
use crate::cli::commands::create::create_template;
use crate::cli::commands::edit::edit_template;
use crate::cli::commands::edit_meta::edit_template_meta;
use crate::cli::commands::list::list_templates;
use crate::cli::commands::output::show_output;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a new template file
    Create { name: String },

    /// Edit the template's prompt using $EDITOR
    Edit { name: String },

    /// Edit the template's meta using $EDITOR
    EditMeta { name: String },

    /// Append a new entry to a multi-value placeholder
    AddToVar {
        /// Name of the template
        name: String,
        /// The placeholder name
        placeholder: String,
        /// Optional file path from which to read placeholder content (defaults to reading from stdin)
        #[arg(long, short = 'f', default_value = "-")]
        file: String,
        /// If set, the placeholder is considered code text in this language
        #[arg(long, short = 'l')]
        language: Option<String>,
        /// If set, find files using `file` as the file extension
        #[arg(long, short = 'd')]
        directory: Option<String>,
        /// Required if `directory` is set to true then acts as a file-extension filter, eg
        /// `.rs` finds all rust source files
        #[arg(long, short = 's')]
        extension: Option<String>,
    },

    /// Print the template with placeholders replaced.
    /// Any `--var KEY=VALUE` arguments here will override placeholders.
    Output {
        name: String,

        /// Overrides in the form KEY=VALUE (applied last).
        #[arg(long = "var", short = 'v', num_args=1.., value_parser = parse_key_val, value_name="KEY=VALUE", value_hint=ValueHint::Other)]
        variables: Vec<(String, String)>,
    },

    /// Clone an existing template to a new name
    Clone {
        /// Name of the existing template to clone from
        source: String,
        /// Name for the new template
        destination: String,
    },

    /// Prints all the available templates
    List,
}

pub fn parse_and_run_cli() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create { name } => create_template(&name)?,
        Commands::Edit { name } => edit_template(&name)?,
        Commands::EditMeta { name } => edit_template_meta(&name)?,
        Commands::AddToVar {
            name,
            placeholder,
            file,
            language,
            directory,
            extension,
        } => add_to_var(&name, &placeholder, &file, language, directory, extension)?,
        Commands::Output { name, variables } => show_output(&name, &variables)?,
        Commands::List => list_templates()?,
        Commands::Clone {
            source,
            destination,
        } => clone_template(&source, &destination)?,
    }

    Ok(())
}

#[derive(Parser, Debug)]
#[clap(version)]
#[command(name = "prompta")]
#[command(about = "A CLI to manage LLM prompts", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    #[arg(long, short = 'c', default_value_t = config_dir_str())]
    pub config_dir: String,
}

/// A small helper to parse `KEY=VALUE` strings into `(KEY, VALUE)` pairs.
fn parse_key_val(s: &str) -> Result<(String, String), String> {
    match s.split_once('=') {
        Some((k, v)) if !k.is_empty() => Ok((k.to_string(), v.to_string())),
        _ => Err(format!("Invalid KEY=VALUE: '{s}'")),
    }
}
