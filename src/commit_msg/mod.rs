mod commit_msg_rule;
mod config;

use colored::Colorize;
pub use commit_msg_rule::CommitMsgRule;
pub use config::ScopeConfig;

// Extract valid first line (skip comments/empty lines)
fn get_commit_msg_first_line(commit_msg: &str) -> &str {
    commit_msg
        .lines()
        .find(|line| !line.trim_start().starts_with('#') && !line.trim().is_empty())
        .unwrap_or_else(|| {
            eprintln!("{}", "Error: Commit message cannot be empty".red());
            std::process::exit(1);
        })
}
