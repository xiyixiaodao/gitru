use std::fs;

use super::commit_msg_rule::CommitMsgRule;

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
    if commit_msg_rule.rules.type_rule.enabled {
        let type_validation_passed = commit_msg_rule
            .rules
            .type_rule
            .config
            .validate_type(&commit_msg);
        if !type_validation_passed {
            eprintln!("Error: Invalid type !");
            std::process::exit(1);
        }
    }
}
