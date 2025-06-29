use colored::Colorize;
use std::{fs, io::Write};
use tracing::{debug, trace};

use crate::config::COMMIT_MSG_HOOK_CONTENT;

/// Install commit-msg hook to .git/hooks directory
pub fn install_commit_msg_hook() {
    trace!(
        "Writing commit-msg hook content: \n{}",
        COMMIT_MSG_HOOK_CONTENT.on_truecolor(64, 64, 64)
    );

    // Get current directory
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    debug!("Current directory: {}", current_dir.display());

    // git repository existence check
    if !current_dir.join(".git").exists() {
        eprintln!("{}", "Error: Not a git repository,init it first".red());
        std::process::exit(1);
    }

    // Build .git/hooks path (cross-platform compatible)
    let git_hooks_path = current_dir.join(".git").join("hooks").join("commit-msg");
    debug!("Hook file path: {}", git_hooks_path.display());

    if !git_hooks_path.exists() {
        // Create parent directories (newly added)
        if let Some(parent) = git_hooks_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create parent directories");
        }
        fs::File::create(&git_hooks_path).expect("Failed to create file");
    }

    // Open file in write mode
    let mut commit_msg = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&git_hooks_path)
        .unwrap();

    // Write content
    // Replace Windows CRLF line endings with Unix LF
    commit_msg
        .write_all(
            COMMIT_MSG_HOOK_CONTENT
                .replace("\r\n", "\n")
                .replace('\r', "")
                .as_bytes(),
        )
        .unwrap();

    // Set file permissions to 755
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let file = fs::File::open(&git_hooks_path).unwrap();
        let mut permissions = file.metadata().unwrap().permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&git_hooks_path, permissions).unwrap();
    }

    println!("{}", "commit-msg hook installed successfully!".green());
}
