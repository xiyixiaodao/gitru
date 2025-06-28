use crate::commit_msg::get_commit_msg_first_line;
use colored::Colorize;
use regex::Regex;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

// Scope validation configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct ScopeConfig {
    pub allowed_scopes: Option<Vec<String>>,
}

impl ScopeConfig {
    pub fn validate_scope(&self, commit_msg: &str) -> bool {
        if self
            .allowed_scopes
            .as_ref()
            .is_none_or(|scopes| scopes.is_empty())
        {
            eprintln!("{}", "allowed scopes is empty".red());
            return false;
        }

        let allowed_scopes = self.allowed_scopes.as_ref().unwrap();

        //avoid allowed_scopes:[""]
        if allowed_scopes.iter().all(|s| s.is_empty()) {
            info!("The allowed_scopes {:?}", allowed_scopes);
            eprintln!(
                "{}",
                "the allowed_scopes cannot be empty, check the yaml file ".red()
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
            eprintln!("{}", "your scope is  empty".blue());
            return false;
        };

        let scope_with_parenthesis = &scope_and_parenthesis_capture["scope"];

        if !scope_with_parenthesis.starts_with("(") {
            eprintln!("{}", "the left parenthesis is missing".blue());
            return false;
        }

        if !scope_with_parenthesis.ends_with(")") {
            eprintln!("{}", "the right parenthesis is missing".blue());
            return false;
        }

        debug!("scope_with_parenthesis:{:?}", scope_with_parenthesis);

        // Remove the left and right parentheses
        let scope_with_parenthesis = scope_with_parenthesis.trim_start_matches("(");
        let scope = scope_with_parenthesis.trim_end_matches(")");

        info!("your scope {:?}", scope);

        if allowed_scopes.contains(&scope.to_owned()) {
            true
        } else {
            eprintln!("{}", format!("your scope :{:?}", scope).blue());
            eprintln!("{}", format!("allowed scopes {:?}", allowed_scopes).blue());
            false
        }
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
        assert!(!scope_config.validate_scope("feat(test123): add feature"));
    }

    #[test]
    fn test_validate_scope2() {
        let scope_config = ScopeConfig {
            allowed_scopes: Some(vec!["test".to_string()]),
        };
        assert!(scope_config.validate_scope("feat(test): test"));
    }

    #[test]
    fn test_validate_scope3() {
        let scope_config = ScopeConfig {
            allowed_scopes: Some(vec!["test".to_string()]),
        };
        assert!(!scope_config.validate_scope("feat(rust): add a new feature"));
    }

    #[test]
    fn test_validate_scope4() {
        let scope_config = ScopeConfig {
            allowed_scopes: Some(vec!["test".to_string()]),
        };
        assert!(!scope_config.validate_scope("feat(): add a new feature"));
        assert!(!scope_config.validate_scope("feat(: add a new feature"));
        assert!(!scope_config.validate_scope("feat): add a new feature"));
        assert!(!scope_config.validate_scope("feat(te: add a new feature"));
    }
}
