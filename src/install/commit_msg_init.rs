use crate::config::{COMMIT_MSG_RULE_CONTENT, COMMIT_MSG_RULE_NAME};
use colored::Colorize;
use std::fs;
use std::io::Write;
use tracing::{debug, trace};

pub fn init_commit_msg_rule() {
    trace!(
        "Writing commit-msg rule content: \n{}",
        COMMIT_MSG_RULE_CONTENT
    );

    // Get current directory
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    debug!("Current directory: {}", current_dir.display());

    // Target file path
    let commit_msg_rule_path = current_dir.join(COMMIT_MSG_RULE_NAME);
    debug!(
        "Target commit-msg rule file path: {}",
        commit_msg_rule_path.display()
    );

    if !commit_msg_rule_path.exists() {
        fs::File::create(&commit_msg_rule_path).expect("Failed to create file");
    }

    // Open file in write mode
    let mut commit_msg = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(commit_msg_rule_path)
        .unwrap();

    // Write content
    commit_msg
        .write_all(COMMIT_MSG_RULE_CONTENT.as_bytes())
        .unwrap();

    println!("{}", "commit-msg rule initialized successfully!".green());
}
