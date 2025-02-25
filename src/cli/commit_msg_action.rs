use std::path::PathBuf;

use clap::Parser;
use colored::Colorize;

#[derive(Parser, Debug)]
pub enum CommitMsgAction {
    /// Initialize config file to project root directory
    // #[command(name = "init")]
    Init,

    /// Install hook to .git/hooks directory
    // #[command(name = "install")]
    Install,

    /// Execute both init and install
    // #[command(name = "ii")]
    II,

    /// Validate if commit message complies with rules
    // #[command(name = "validate")]
    Validate {
        /// Temporary storage path for user's commit message
        #[arg(long, value_parser = validate_file_path)]
        msg_path: String,

        /// Validation rule file path for commit-msg
        #[arg(long, value_parser = validate_file_path)]
        rule_path: String,
    },
}

fn validate_file_path(s: &str) -> Result<String, String> {
    let path = PathBuf::from(s);
    if !path.exists() {
        return Err(format!("File {} does not exist!", s.red()));
    }
    Ok(s.to_owned())
}
