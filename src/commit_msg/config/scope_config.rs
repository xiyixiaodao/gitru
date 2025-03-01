use colored::Colorize;
use serde::{Deserialize, Serialize};

// Scope validation configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct ScopeConfig {
    pub allow_empty: bool,
    pub allow_custom_scopes: bool,
    pub allowed_scopes: Vec<String>,
}

impl ScopeConfig {
    pub fn validate_scope(&self, commit_msg: &str) -> bool {
        // Extract valid first line (skip comments/empty lines)
        let first_line = commit_msg
            .lines()
            .find(|line| !line.trim_start().starts_with('#') && !line.trim().is_empty())
            .unwrap_or_else(|| {
                eprintln!("{}", "Error: Commit message cannot be empty".red());
                std::process::exit(1);
            });

        // Split type declaration and subject
        let type_scope_subject: Vec<&str> = first_line.splitn(2, ':').collect();

        // When validating scope, type has already been validated
        let type_scope = type_scope_subject.first().unwrap();

        // Lone '(' or ')' already handled in type validation
        // `()` present
        if type_scope.contains('(') && type_scope.contains(')') {
            let scope = type_scope.split('(').nth(1).unwrap().split(')').next();

            if self.allow_empty {
                // If scope is missing and allow_empty is true
                if scope.is_none() {
                    true
                } else {
                    // Scope exists, custom scopes allowed
                    if self.allow_custom_scopes {
                        // Non-empty is acceptable
                        !scope.unwrap().trim().is_empty()
                    } else {
                        // Custom scopes not allowed, scope must be in allowed_scopes
                        if !self.allowed_scopes.contains(&scope.unwrap().to_string()) {
                            eprintln!(
                                "{}",
                                format!("Error: Scope '{}' is not allowed", scope.unwrap()).red()
                            );
                            eprintln!("Allowed scopes:");
                            eprintln!("{}", self.allowed_scopes.join("\n").green());
                            return false;
                        }
                        true
                    }
                }
            } else {
                // Scope must be present

                // Scope exists, custom scopes allowed
                if self.allow_custom_scopes {
                    // Non-empty is acceptable
                    !scope.unwrap().trim().is_empty()
                } else {
                    // Custom scopes not allowed, scope must be in allowed_scopes
                    if !self.allowed_scopes.contains(&scope.unwrap().to_string()) {
                        eprintln!(
                            "{}",
                            format!("Error: Scope '{}' is not allowed", scope.unwrap()).red()
                        );
                        eprintln!("Allowed scopes:");
                        eprintln!("{}", self.allowed_scopes.join("\n").green());
                        return false;
                    }
                    true
                }
            }
        } else {
            // `()` missing, no scope, allow_empty is true
            if self.allow_empty {
                true
            } else {
                eprintln!("{}", "Error: Scope is required".red());
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_scope1() {
        let scope_config = ScopeConfig {
            allow_empty: false,
            allow_custom_scopes: false,
            allowed_scopes: vec!["test".to_string()],
        };
        assert!(!scope_config.validate_scope("feat(test123): test"));
    }

    #[test]
    fn test_validate_scope2() {
        let scope_config = ScopeConfig {
            allow_empty: false,
            allow_custom_scopes: false,
            allowed_scopes: vec!["test".to_string()],
        };
        assert!(scope_config.validate_scope("feat(test): test"));
    }

    #[test]
    fn test_validate_scope3() {
        let scope_config = ScopeConfig {
            allow_empty: true,
            allow_custom_scopes: false,
            allowed_scopes: vec!["test".to_string()],
        };
        assert!(!scope_config.validate_scope("feat(java): add a new feature"));
    }
}
