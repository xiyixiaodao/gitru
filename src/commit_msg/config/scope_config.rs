use crate::commit_msg::get_commit_msg_first_line;
use crate::config::COMMIT_MSG_RULE_NAME;
use colored::Colorize;
use regex::Regex;
use serde::{Deserialize, Serialize};
use tracing::field::debug;
use tracing::{debug, info};

// Scope validation configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct ScopeConfig {
    pub allowed_scopes: Option<Vec<String>>,
}

impl ScopeConfig {
    pub fn validate_scope_with_config(&self, commit_msg: &str) -> bool {
        if self
            .allowed_scopes
            .as_ref()
            .is_none_or(|scopes| scopes.is_empty())
        {
            eprintln!(
                "{}",
                format!("allowed scopes is empty in the file: {COMMIT_MSG_RULE_NAME}")
                    .bright_blue()
            );
            return false;
        }

        let allowed_scopes = self.allowed_scopes.as_ref().unwrap();

        //avoid allowed_scopes:[""]
        if allowed_scopes.iter().all(|s| s.is_empty()) {
            info!("The allowed_scopes {:?}", allowed_scopes);
            eprintln!(
                "{}",
                format!("allowed_scopes is empty, check the file: {COMMIT_MSG_RULE_NAME}")
                    .bright_blue()
            );
            return false;
        }

        // Extract valid first line (skip comments/empty lines)
        let first_line = get_commit_msg_first_line(commit_msg);

        let allowed_scopes = self.allowed_scopes.as_ref().unwrap();

        // feat(): add a new feature .  is not allowed
        let regex = Regex::new(r"^[^()]*(?<scope>[(|)][^:]*):.*").unwrap();
        debug!("scope capture: {:?}", regex.captures(first_line));

        let Some(scope_and_parenthesis_capture) = regex.captures(first_line) else {
            info!("allowed scope: {:?}", allowed_scopes);
            eprintln!("{}", "your scope is  empty".bright_blue());
            return false;
        };

        let scope_with_parenthesis = &scope_and_parenthesis_capture["scope"];

        if !scope_with_parenthesis.starts_with("(") {
            eprintln!(
                "{}",
                "the ‘scope’ left parenthesis is missing".bright_blue()
            );
            return false;
        }

        if !scope_with_parenthesis.ends_with(")") {
            eprintln!(
                "{}",
                "the 'scope' right parenthesis is missing".bright_blue()
            );
            return false;
        }

        debug!("scope_with_parenthesis:{:?}", scope_with_parenthesis);

        // Remove the first left and first right  parentheses
        let scope_without_left_paren = scope_with_parenthesis
            .strip_prefix("(")
            .unwrap_or(scope_with_parenthesis);
        let scope = scope_without_left_paren
            .strip_suffix(")")
            .unwrap_or(scope_without_left_paren);

        info!("your scope {:?}", scope);

        if allowed_scopes.contains(&scope.to_owned()) {
            true
        } else {
            eprintln!("{}", format!("your scope : {scope:?}").bright_blue());
            eprintln!(
                "{}",
                format!("allowed scopes : {allowed_scopes:?}").bright_blue()
            );
            false
        }
    }

    //scope syntax rules are checked to detect only parentheses matching
    pub fn validate_scope_without_config(commit_msg: &str) -> bool {
        // Extract valid first line (skip comments/empty lines)
        let first_line = get_commit_msg_first_line(commit_msg);

        let regex = Regex::new(r"^[^()]*(?<scope>[(|)][^:]*):.*").unwrap();
        debug!("scope capture: {:?}", regex.captures(first_line));

        let Some(scope_and_parenthesis_capture) = regex.captures(first_line) else {
            debug("scope capture is empty");
            return true;
        };

        let scope_with_parenthesis = &scope_and_parenthesis_capture["scope"];

        if !scope_with_parenthesis.starts_with("(") {
            eprintln!(
                "{}",
                "the ‘scope’ left parenthesis is missing".bright_blue()
            );
            return false;
        }

        if !scope_with_parenthesis.ends_with(")") {
            eprintln!(
                "{}",
                "the 'scope' right parenthesis is missing".bright_blue()
            );
            return false;
        }

        debug!("scope_with_parenthesis:{:?}", scope_with_parenthesis);

        // remove only first left and first right  parentheses
        let scope_without_left_paren = scope_with_parenthesis
            .strip_prefix("(")
            .unwrap_or(scope_with_parenthesis);
        let scope = scope_without_left_paren
            .strip_suffix(")")
            .unwrap_or(scope_without_left_paren);

        info!("your scope {:?}", scope);

        // feat(): add a new feature.
        if scope.is_empty() {
            eprintln!(
                "{}",
                "when scope is empty , ( ) is not needed".bright_blue()
            );
            return false;
        }

        if scope.contains("(") || scope.contains(")") {
            eprintln!(
                "{}",
                "the syntax of 'scope' is incorrect, contain redundant parentheses".bright_blue()
            );
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_scope1() {
        let scope_config = ScopeConfig {
            allowed_scopes: Some(vec!["test".to_string()]),
        };
        assert!(!scope_config.validate_scope_with_config("feat(test123): add feature"));
    }

    #[test]
    fn test_validate_scope2() {
        let scope_config = ScopeConfig {
            allowed_scopes: Some(vec!["test".to_string()]),
        };
        assert!(scope_config.validate_scope_with_config("feat(test): test"));
    }

    #[test]
    fn test_validate_scope3() {
        let scope_config = ScopeConfig {
            allowed_scopes: Some(vec!["test".to_string()]),
        };
        assert!(!scope_config.validate_scope_with_config("feat(rust): add a new feature"));
    }

    #[test]
    fn test_validate_scope4() {
        let scope_config = ScopeConfig {
            allowed_scopes: Some(vec!["test".to_string()]),
        };
        assert!(!scope_config.validate_scope_with_config("feat(): add a new feature"));
        assert!(!scope_config.validate_scope_with_config("feat(: add a new feature"));
        assert!(!scope_config.validate_scope_with_config("feat): add a new feature"));
        assert!(!scope_config.validate_scope_with_config("feat(te: add a new feature"));
    }
}
