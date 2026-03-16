use crate::error::git_status::ConfigStatusCheckError;
use crate::util::colored_print::print_warning;
use std::process::Command;

/// Run a Git command and return its stdout as a String.
/// Git commands must use paths relative to the repository root.
/// Passing absolute paths will cause Git to misinterpret file status.
fn run_git(args: &[&str]) -> Result<String, ConfigStatusCheckError> {
    let output = Command::new("git")
        .args(args)
        .output()
        .map_err(|e| ConfigStatusCheckError::GitError(e.to_string()))?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Retrieve the Git status of a file using `git status --porcelain`.  
/// The porcelain format provides two status columns:  
///   X = index (staged) status  
///   Y = working tree (unstaged) status  
///
/// Examples:  
///   " M file" → X=' ', Y='M'  (modified but not staged)  
///   "M  file" → X='M', Y=' '  (staged modification)  
///   "MM file" → X='M', Y='M'  (partially staged)  
///   "?? file" → untracked  
///
/// Returns (' ', ' ') if the file does not appear in the porcelain output.  
fn get_git_status(path: &str) -> Result<(char, char), ConfigStatusCheckError> {
    let out = run_git(&["status", "--porcelain"])?;
    for line in out.lines() {
        if line.len() < 3 {
            continue;
        }
        let x = line.chars().next().unwrap();
        let y = line.chars().nth(1).unwrap();
        let file = &line[3..];

        if file == path {
            return Ok((x, y));
        }
    }
    Ok((' ', ' ')) // Clean: no staged or unstaged changes
}

/// Validate the Git status of the configuration file.
///
/// Rules:
/// - Untracked file → warn but allow commit
/// - Modified but not staged (Y == 'M') → block commit
/// - All other states → allow commit
///
/// This ensures users do not accidentally commit with unstaged config changes,
/// including partially staged modifications.
pub fn check_config_status(config: &str) -> Result<(), ConfigStatusCheckError> {
    // Ensure the file exists before checking Git status.
    if !std::path::Path::new(config).exists() {
        return Err(ConfigStatusCheckError::ConfigNotExist(config.to_string()));
    }

    let (x, y) = get_git_status(config)?;

    // Untracked file: warn but do not block the commit.
    // Users may intentionally add the config file later.
    if x == '?' && y == '?' {
        print_warning(&format!(
            "warning: configuration file '{}' exists but is not tracked by git. \
             It is recommended to add it to version control.",
            config
        ));
        return Ok(());
    }

    // Working tree modification not staged (Y == 'M'):
    // This includes partially staged changes (e.g., X='M', Y='M').
    // Block the commit to prevent silently ignoring new config changes.
    if y == 'M' {
        return Err(ConfigStatusCheckError::ConfigNotCommitted {
            file: config.to_string(),
        });
    }

    // All other states (clean, staged-only, etc.) → allow commit.
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::constant::COMMIT_MSG_RULE_FILE_NAME;
    use crate::validator::git_status::check_config_status;

    #[test]
    fn test_check_config_status() {
        let file = COMMIT_MSG_RULE_FILE_NAME;
        println!("{:?}", check_config_status(file));
    }
}
