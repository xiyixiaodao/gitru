use std::fs;

use colored::Colorize;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CommitConfig {
    pub rules: CommitRules,
}

#[derive(Debug, Deserialize)]
pub struct CommitRules {
    pub allowed_types: Vec<String>,
}

pub fn validate_msg(msg_path: &str, rule_path: &str) {
    // Read user's commit message
    let msg = fs::read_to_string(msg_path).unwrap_or_else(|_| {
        eprintln!("Error: Failed to read commit message file {}", msg_path);
        std::process::exit(1);
    });

    // Read validation rules
    let rule_content = fs::read_to_string(rule_path).unwrap_or_else(|_| {
        eprintln!("Error: Failed to read rule file {}", rule_path);
        std::process::exit(1);
    });

    let config: CommitConfig = serde_yaml::from_str(&rule_content).unwrap();

    // Filter comments (allow spaces before #)
    let first_line = msg
        .lines()
        .find(|line| !line.trim_start().starts_with('#') && !line.trim().is_empty())
        .unwrap_or_else(|| {
            eprintln!("Error: Commit message cannot be empty");
            std::process::exit(1);
        });

    // Validate message format
    let parts: Vec<&str> = first_line.splitn(2, ':').collect();
    if parts.len() < 2 {
        eprintln!(
            "{}",
            "Error: Invalid commit message format, missing type separator ':'".bright_red()
        );
        eprintln!(
            "{}",
            "Valid format example: feat: Add new feature".bright_cyan()
        );
        std::process::exit(1);
    }

    let type_part = parts[0].trim();
    if type_part.is_empty() {
        eprintln!(
            "{}",
            "Error: Type declaration required before colon".bright_red()
        );
        eprintln!(
            "{}",
            "Valid format example: feat: Add new feature".bright_cyan()
        );
        std::process::exit(1);
    }
    let allowed_types = config.rules.allowed_types;

    // Format error messages
    if !allowed_types.contains(&type_part.to_string()) {
        let e = format!("\nError: Commit type '{}' not allowed", type_part).red();
        eprintln!("{}", e);
        eprintln!("Allowed types:\n{}", allowed_types.join("\n").green());
        std::process::exit(1);
    }
}
