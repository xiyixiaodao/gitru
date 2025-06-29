use colored::Colorize;
use tracing::debug;

pub fn remove_commit_msg_hook() {
    // Get current directory
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    debug!("Current directory: {}", current_dir.display());

    // git repository existence check
    if !current_dir.join(".git").exists() {
        eprintln!("{}", "Error: Not a git repository".red());
        std::process::exit(1);
    }

    // Build .git/hooks path (cross-platform compatible)
    let git_hooks_path = current_dir.join(".git").join("hooks").join("commit-msg");
    debug!("Hook file path: {}", git_hooks_path.display());

    if !git_hooks_path.exists() {
        eprintln!("{}", "did not find a commit-msg hook".red());
        std::process::exit(1);
    } else {
        match std::fs::remove_file(&git_hooks_path) {
            Ok(_) => {
                println!("{}", "commit-msg hook removed successfully!".green());
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("{}", format!("Failed to remove commit-msg hook: {e}").red());
                std::process::exit(1);
            }
        }
    }
}
