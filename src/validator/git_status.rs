use crate::error::git_status::ConfigStatusCheckError;
use crate::util::colored_print::print_warning;
use std::process::Command;

// Git uses paths relative to the repository root. You should NOT pass an
// absolute or full filesystem path; otherwise Git cannot correctly determine
// the file's status.

fn run_git(args: &[&str]) -> Result<String, ConfigStatusCheckError> {
    let output = Command::new("git")
        .args(args)
        .output()
        .map_err(|e| ConfigStatusCheckError::GitError(e.to_string()))?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn is_untracked(path: &str) -> Result<bool, ConfigStatusCheckError> {
    let out = run_git(&["ls-files", "--others", "--exclude-standard"])?;
    Ok(out.lines().any(|line| line == path))
}

fn is_staged(path: &str) -> Result<bool, ConfigStatusCheckError> {
    let out = run_git(&["diff", "--cached", "--name-only"])?;
    Ok(out.lines().any(|line| line == path))
}

fn is_modified(path: &str) -> Result<bool, ConfigStatusCheckError> {
    let out = run_git(&["diff", "--name-only"])?;
    Ok(out.lines().any(|line| line == path))
}

/// Check the status of the configuration file.
///
/// - Untracked → warn but do not block the commit
/// - Modified but not staged → block the commit
/// - All other cases → allow the commit
pub fn check_config_status(config: &str) -> Result<(), ConfigStatusCheckError> {
    // Check whether the file exists
    if !std::path::Path::new(config).exists() {
        return Err(ConfigStatusCheckError::ConfigNotExist(config.to_string()));
    }

    let untracked = is_untracked(config)?;
    let staged = is_staged(config)?;
    let modified = is_modified(config)?;

    // Case 1: untracked → warn but do not block
    if untracked {
        print_warning(
            format!(
                "warning: configuration file '{}' exists but is not tracked by git. \
                 It is recommended to add it to version control.",
                config
            )
            .as_str(),
        );
        return Ok(());
    }

    // Case 2: tracked but modified and not staged → block
    if modified && !staged {
        return Err(ConfigStatusCheckError::ConfigNotCommitted {
            file: config.to_string(),
        });
    }

    // Case 3: all other cases → allow
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
