pub mod add_to_var;
pub mod clone;
pub mod create;
pub mod edit;
pub mod edit_meta;
pub mod list;
pub mod output;

pub fn check_dir_and_ext_args_together(directory: &Option<String>, extension: &Option<String>) {
    if let Some(_) = directory {
        if let Some(_) = extension {
            // All good
        } else {
            panic!("If --directory is provided, --extension is also required")
        }
    }
}
