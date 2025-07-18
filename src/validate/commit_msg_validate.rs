use std::fs;

use crate::commit_msg;
use crate::commit_msg::CommitMsgRule;
use colored::Colorize;
use tracing::info;

pub fn get_commit_msg_rule(rule_path: &str) -> String {
    fs::read_to_string(rule_path).unwrap_or_else(|_| {
        eprintln!("Error: Failed to read rule file commit_msg_rule.yaml");
        std::process::exit(1);
    })
}

fn get_commit_msg(msg_path: &str) -> String {
    fs::read_to_string(msg_path).unwrap_or_else(|_| {
        eprintln!("Error: Failed to read commit message file {msg_path}");
        std::process::exit(1);
    })
}

pub fn validate_msg(msg_path: &str, rule_path: &str) {
    println!("\ncommit-msg validation result: ");

    let commit_msg = get_commit_msg(msg_path);
    let rule_content = get_commit_msg_rule(rule_path);

    let commit_msg_rule: CommitMsgRule = serde_yaml::from_str(&rule_content)
        .expect("Failed to parse .commit-msg-rule.yaml; check the file format");

    if commit_msg_rule.rules.is_none() {
        eprintln!("{}", "Error: commit-msg-rule.yaml contains no rules".red());
        std::process::exit(1);
    }

    let mut type_validation_passed = true;
    let scope_validation_passed;
    let mut subject_validation_passed = true;

    // Validate commit-message

    // Validate type
    if let Some(type_config) = &commit_msg_rule.rules.as_ref().unwrap().type_rule {
        info!("type_config:{:?}", type_config);

        type_validation_passed = type_config.validate_type(&commit_msg);
        if !type_validation_passed {
            eprintln!("{}", "Type validation failed!".red());
        }
    }

    // Validate scope with config
    if let Some(scope_config) = &commit_msg_rule.rules.as_ref().unwrap().scope {
        info!("scope_config:{:?}", scope_config);

        scope_validation_passed = scope_config.validate_scope_with_config(&commit_msg);
        if !scope_validation_passed {
            eprintln!("{}", "Scope validation failed!".red());
        }
    } else {
        // Validate scope without config
        scope_validation_passed =
            commit_msg::ScopeConfig::validate_scope_without_config(&commit_msg);
        if !scope_validation_passed {
            eprintln!("{}", "Scope validation failed!".red());
        }
    }

    // Validate subject
    if let Some(subject_config) = &commit_msg_rule.rules.as_ref().unwrap().subject {
        info!("subject_config:{:?}", subject_config);

        subject_validation_passed = subject_config.validate_subject(&commit_msg);
        if !subject_validation_passed {
            eprintln!("{}", "Subject validation failed!".red());
        }
    }

    if type_validation_passed && scope_validation_passed && subject_validation_passed {
        println!("{}", "Commit message validation successful!".green());
        std::process::exit(0);
    } else {
        eprintln!("\n{}", "Commit message validation failed, please modify the message according to the above prompts".red());
        eprintln!("\na regular commit-msg may like: ");
        eprintln!("{}", "type: subject".green());
        eprintln!("OR");
        eprintln!("{}", "type(scope): subject".green());
        std::process::exit(1);
    }
}
