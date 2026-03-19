use crate::config::commit_msg_rule::parse_commit_msg_rule;
use crate::constant::{
    COMMIT_MSG_HOOK_CONTENT, COMMIT_MSG_RULE_FILE_NAME, COMMIT_MSG_RULE_TEMPLATE,
};
use crate::parser::commit_msg::parse_commit_msg;
use crate::parser::get_first_non_empty_line;
use crate::util::colored_print::{print_error, print_success, print_warning};
use crate::util::git_path::detect_current_repo;
use crate::validator::commit_msg::validate_commit_msg;
use crate::validator::git_status::check_config_status;
use std::fs;
use std::path::PathBuf;

pub fn init(force: bool) -> Result<(), String> {
    let git_kind =
        detect_current_repo().map_err(|e| format!("failed to detect git kind: {}", e))?;
    let path = git_kind.config_path(COMMIT_MSG_RULE_FILE_NAME);
    if path.exists() && !force {
        return Err(format!(
            "commit-msg config file '{}' already exists at {}. Use -f or --force to overwrite.",
            COMMIT_MSG_RULE_FILE_NAME,
            path.display()
        ));
    }

    if let Err(e) = fs::write(&path, COMMIT_MSG_RULE_TEMPLATE) {
        print_error(&format!("unable to write commit-msg config file: {}", e));
        std::process::exit(1);
    }

    print_success(&format!(
        "commit-msg config file '{}' has been initialized at {}",
        COMMIT_MSG_RULE_FILE_NAME,
        path.display()
    ));
    Ok(())
}

pub fn install(force: bool) -> Result<(), String> {
    let git_kind =
        detect_current_repo().map_err(|e| format!("failed to detect git kind: {}", e))?;
    let hook_path = git_kind.hook_path("commit-msg");

    if hook_path.exists() && !force {
        print_error(&format!(
            "commit-msg hook already exists at {}. Use -f or --force to overwrite.",
            hook_path.display()
        ));
        std::process::exit(1);
    }

    if let Err(e) = fs::write(&hook_path, COMMIT_MSG_HOOK_CONTENT.as_str()) {
        print_error(&format!("unable to write commit-msg hook: {}", e));
        std::process::exit(1);
    }

    #[cfg(target_family = "unix")]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&hook_path, fs::Permissions::from_mode(0o755))
            .map_err(|e| format!("unable to set commit-msg hook permissions: {}", e))?;
    }

    print_success(&format!(
        "commit-msg hook has been installed at {}",
        hook_path.display()
    ));
    Ok(())
}

pub fn uninstall() -> Result<(), String> {
    let git_kind =
        detect_current_repo().map_err(|e| format!("failed to detect git kind: {}", e))?;
    let hook_path = git_kind.hook_path("commit-msg");

    if hook_path.exists() {
        if let Err(e) = fs::remove_file(&hook_path) {
            return Err(format!("unable to remove commit-msg hook: {}", e));
        }
        print_success(&format!(
            "commit-msg hook has been uninstalled at {}",
            hook_path.display()
        ));
    } else {
        print_warning("unable to find commit-msg hook");
    }

    Ok(())
}

pub fn run(msg_path: &PathBuf, rule_path: &PathBuf) -> Result<(), String> {
    // Git uses paths relative to the repository root. You should NOT pass an
    // absolute or full filesystem path; otherwise Git cannot correctly determine
    // the file's status.
    let relative_path = COMMIT_MSG_RULE_FILE_NAME;
    check_config_status(relative_path).map_err(|e| e.to_string())?;

    // Parse and validate the commit message rule file
    let rule_content =
        fs::read_to_string(rule_path).map_err(|e| format!("cannot read commit message: {}", e))?;
    let parsed_rule = parse_commit_msg_rule(&rule_content).map_err(|e| e.to_string())?;

    // Global validation switch
    let enable_validation = parsed_rule
        .global
        .as_ref()
        .and_then(|g| g.enable_validation)
        .unwrap_or(true);

    // Only when the global switch is explicitly set to false, return Ok
    if !enable_validation {
        return Ok(());
    }

    let commit_msg_raw =
        fs::read_to_string(msg_path).map_err(|e| format!("cannot read commit message: {}", e))?;

    if let Some(first_line) = get_first_non_empty_line(&commit_msg_raw) {
        // Get skip_validation_words, default is an empty list
        let skip_words = parsed_rule
            .global
            .as_ref()
            .and_then(|g| g.skip_validation_words.clone())
            .unwrap_or_else(Vec::new);

        // Check if the first line matches any skip words
        if skip_words
            .iter()
            .any(|w| w.eq_ignore_ascii_case(first_line.trim()))
        {
            return Ok(());
        }
    } else {
        return Err("commit message cannot be empty".to_string());
    }

    let msg = parse_commit_msg(&commit_msg_raw);
    let msg = msg.map_err(|e| e.to_string())?;

    // Validate commit message against the configured rules
    let is_valid = validate_commit_msg(&msg, &parsed_rule);
    if let Err(e) = is_valid {
        // Note: The error message here will be printed directly to stderr for the user, do not use debug mode
        return Err(format!("error: {}", e));
    }

    Ok(())
}
