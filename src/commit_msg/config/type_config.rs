use crate::commit_msg::get_commit_msg_first_line;
use colored::Colorize;
use regex::Regex;
use serde::{Deserialize, Serialize};
use tracing::info;

// Type validation configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct TypeConfig {
    pub allowed_types: Option<Vec<String>>,
}

impl TypeConfig {
    pub fn validate_type(&self, commit_msg: &str) -> bool {
        if self
            .allowed_types
            .as_ref()
            .is_none_or(|types| types.is_empty())
        {
            eprintln!("{}", "allowed types is empty".red());
            return false;
        }

        // Extract valid first line (skip comments/empty lines)
        let first_line = get_commit_msg_first_line(commit_msg);

        let allowed_types = self.allowed_types.as_ref().unwrap();

        // Avoid having empty string items in the `allowed_types` array. eg: allowed_types:[""]
        // When configuring `allowed_types` in a YAML file, this situation can occur if it is written as follows:
        // allowed_types:
        //     -
        //     -
        if allowed_types.iter().all(|t| t.is_empty()) {
            info!("The allowed_types  {:?}", allowed_types);
            eprintln!(
                "{}",
                "the allowed_types cannot be empty, check the yaml file ".red()
            );
            return false;
        }

        let regex = Regex::new(r"^(?<type>[^()]+)(\(?[^:]*)?:.*").unwrap();
        let Some(type_capture) = regex.captures(first_line) else {
            eprintln!("no type found");
            return false;
        };

        let typ = &type_capture["type"];
        info!("your type: {:?}", typ);

        if allowed_types.contains(&typ.to_owned()) {
            true
        } else {
            eprintln!("{}", format!("your type : {typ:?}").blue());
            eprintln!("{}", format!("allowed_types :{allowed_types:?}").blue());
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_type_success() {
        let commit_msg = "feat: Add new feature";
        let type_config = TypeConfig {
            allowed_types: Some(vec!["feat".to_owned(), "fix".to_owned()]),
        };

        assert!(type_config.validate_type(commit_msg));
    }

    #[test]
    fn test_validate_type_fail() {
        let commit_msg = "fix: fix bug";
        let type_config = TypeConfig {
            allowed_types: Some(vec!["feat".to_owned(), "doc".to_owned()]),
        };

        assert!(!type_config.validate_type(commit_msg));
    }
}
