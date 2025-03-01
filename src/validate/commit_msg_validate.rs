use std::fs;

use colored::Colorize;

use crate::commit_msg::CommitMsgRule;

pub fn get_commit_msg_rule(rule_path: &str) -> String {
    fs::read_to_string(rule_path).unwrap_or_else(|_| {
        eprintln!("Error: Failed to read rule file commit_msg_rule.yaml");
        std::process::exit(1);
    })
}

fn get_commit_msg(msg_path: &str) -> String {
    fs::read_to_string(msg_path).unwrap_or_else(|_| {
        eprintln!("Error: Failed to read commit message file {}", msg_path);
        std::process::exit(1);
    })
}

pub fn validate_msg(msg_path: &str, rule_path: &str) {
    let commit_msg = get_commit_msg(msg_path);
    let rule_content = get_commit_msg_rule(rule_path);

    let commit_msg_rule: CommitMsgRule = serde_yaml::from_str(&rule_content).unwrap();

    // Validate commit message
    // Validate type
    if commit_msg_rule.rules.type_rule.enabled {
        let type_validation_passed = commit_msg_rule
            .rules
            .type_rule
            .config
            .validate_type(&commit_msg);
        if !type_validation_passed {
            eprintln!("{}", "Type validation failed!".red());
            std::process::exit(1);
        } else {
            println!("{}", "Type validation passed!".green());
        }
    }

    // Validate scope
    if commit_msg_rule.rules.scope.enabled {
        let scope_validation_passed = commit_msg_rule
            .rules
            .scope
            .config
            .validate_scope(&commit_msg);
        if !scope_validation_passed {
            eprintln!("{}", "Scope validation failed!".red());
            std::process::exit(1);
        } else {
            println!("{}", "Scope validation passed!".green());
        }
    }
}
