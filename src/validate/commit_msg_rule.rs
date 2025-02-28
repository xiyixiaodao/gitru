use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CommitMsgRule {
    pub rules: RuleSet,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RuleSet {
    // type is the rust keyword,type_rule is used instead of type in yaml
    #[serde(rename = "type")]
    pub type_rule: ValidationRule<TypeConfig>,
    pub scope: ValidationRule<ScopeConfig>,
    pub subject: ValidationRule<SubjectConfig>,
    pub body: ValidationRule<BodyConfig>,
    pub footer: ValidationRule<FooterConfig>,
}

// Generic validation rule structure
#[derive(Debug, Deserialize, Serialize)]
pub struct ValidationRule<T> {
    pub enabled: bool,
    pub config: T,
}

// Type validation configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct TypeConfig {
    pub allow_custom_types: bool,
    pub allowed_types: Vec<String>,
}

// Scope validation configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct ScopeConfig {
    pub allow_empty: bool,
    pub allow_custom_scopes: bool,
    pub allowed_scopes: Vec<String>,
}

// Subject validation configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct SubjectConfig {
    pub pre_whitespace: bool,
    pub min_length: u8,
    pub max_length: u8,
}

// Body validation configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct BodyConfig {
    pub blank_line_at_start: bool,
    pub blank_lines_number: u8,
    pub max_line_length: u8,
}

// Footer validation configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct FooterConfig {
    pub allowed_keys: Vec<String>,
}

impl TypeConfig {
    pub fn validate_type(&self, commit_msg: &str) -> bool {
        // Extract valid first line (skip comments/empty lines)
        let first_line = commit_msg
            .lines()
            .find(|line| !line.trim_start().starts_with('#') && !line.trim().is_empty())
            .unwrap_or_else(|| {
                eprintln!("Error: Commit message cannot be empty");
                std::process::exit(1);
            });

        // Split type declaration and subject
        let type_and_subject: Vec<&str> = first_line.splitn(2, ':').collect();

        if type_and_subject.len() < 2 {
            //Error handling: Missing colon separator
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

        // Part before colon
        let type_segment = type_and_subject[0];
        if type_segment.is_empty() {
            //Error handling: Empty type declaration
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

        // Allow custom types, return directly
        if self.allow_custom_types {
            return true;
        }

        // Possible formats (only care about before colon part)
        // type: ...
        // type!: ...
        // type(scope):...
        // type(scope)!:...

        let scope_component: Vec<&str> = type_segment.split('(').collect();
        if scope_component.len() == 1 {
            // type: ...
            // type!: ...
            let base_type = type_segment.trim_end_matches('!');
            // Check whether the type is in the list of allowed types
            if !self.allowed_types.contains(&base_type.to_owned()) {
                eprintln!(
                    "{}",
                    format!(
                        "\nError: Commit type '{}' not allowed, \nAllowed types:\n{}",
                        base_type.red(),
                        self.allowed_types.join("\n").green()
                    )
                    .bright_red()
                );
                return false;
            }
        }

        if scope_component.len() == 2 {
            //type(:...    not allowed
            //type():      not allowed
            //type(scope:  not allowed
            let scope_component = scope_component[1];
            if scope_component.is_empty()
                || scope_component == ")"
                || !scope_component.contains(")")
            {
                let e = format!("\nError: Commit type '{}' not allowed", type_segment).red();
                eprintln!("{}", e);
                eprintln!("Allowed types:\n{}", self.allowed_types.join("\n").green());
                return false;
            }

            //The validation of the content of Scope itself is handled by the ScopeConfig.
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_type_success() {
        let commit_msg = "feat: Add new feature";
        let commit_msg2 = "fix!(lang): Add new language ";
        let type_config = TypeConfig {
            allow_custom_types: false,
            allowed_types: vec!["feat".to_owned(), "fix".to_owned()],
        };

        assert!(type_config.validate_type(commit_msg));
        assert!(type_config.validate_type(commit_msg2));
    }

    #[test]
    fn test_validate_type_fail() {
        let commit_msg = "feat(): Add new feature";
        let commit_msg2 = "feat(: Add new feature";
        let commit_msg3 = "feat!(): Add new feature";
        let commit_msg4 = "feat!(: Add new feature";
        let type_config = TypeConfig {
            allow_custom_types: false,
            allowed_types: vec!["feat".to_owned(), "fix".to_owned()],
        };
        assert!(!type_config.validate_type(commit_msg));
        assert!(!type_config.validate_type(commit_msg2));
        assert!(!type_config.validate_type(commit_msg3));
        assert!(!type_config.validate_type(commit_msg4));
    }
}
