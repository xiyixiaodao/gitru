use crate::commit_msg::get_commit_msg_first_line;
use colored::Colorize;

use regex::Regex;
use serde::{Deserialize, Serialize};
use tracing::info;

// Subject validation configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct SubjectConfig {
    pub require_space_after_colon: Option<bool>,
    pub min_length: Option<u8>,
    pub max_length: Option<u8>,
}

impl SubjectConfig {
    pub fn validate_subject(&self, commit_msg: &str) -> bool {
        let require_space_after_colon = self.require_space_after_colon.unwrap_or(true);

        // Extract valid first line (skip comments/empty lines)
        let first_line = get_commit_msg_first_line(commit_msg);

        let regex = Regex::new(r"^.+:(?<subject>.+)").unwrap();

        let Some(subject_capture) = regex.captures(first_line) else {
            eprintln!("{}", "no subject found".bright_blue());
            return false;
        };

        let subject = &subject_capture["subject"];
        info!("your subject: {:?}", subject);

        // Whether a space is required
        if require_space_after_colon && !subject.starts_with(" ") {
            eprintln!("{}", "subject need space in the first".bright_blue());
            return false;
        } else if !require_space_after_colon && subject.starts_with(" ") {
            eprintln!(
                "{}",
                "subject did not need space in the first".bright_blue()
            );
            return false;
        }

        let min_length = self.min_length.unwrap_or(1) as usize;
        let max_length = self.max_length.unwrap_or(72) as usize;

        // Count of non-Latin characters
        let subject_length = subject.trim().chars().count();

        if subject_length >= min_length && subject_length <= max_length {
            return true;
        } else {
            eprintln!(
                "{}",
                format!(
                    "subject length [{subject_length}] is not within the specified range:{min_length}————{max_length}"
                )
                .bright_blue()
            )
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_subject_space() {
        let commit_msg = "feat: Add new feature";
        let subject_config = SubjectConfig {
            require_space_after_colon: Some(true),
            min_length: Some(10),
            max_length: Some(50),
        };
        assert!(subject_config.validate_subject(commit_msg));
    }

    #[test]
    fn test_validate_subject_space_failure() {
        let commit_msg = "feat:Add new feature";
        let subject_config = SubjectConfig {
            require_space_after_colon: Some(true),
            min_length: Some(10),
            max_length: Some(50),
        };
        assert!(!subject_config.validate_subject(commit_msg));
    }

    #[test]
    fn test_validate_subject_space_failure2() {
        let commit_msg = "feat: Add new feature";
        let subject_config = SubjectConfig {
            require_space_after_colon: Some(false),
            min_length: Some(10),
            max_length: Some(50),
        };
        assert!(!subject_config.validate_subject(commit_msg));
    }
    #[test]
    fn test_validate_subject_length_failure() {
        let commit_msg = "subject: This is a very long subject that exceeds the maximum length";
        let subject_config = SubjectConfig {
            require_space_after_colon: Some(true),
            min_length: Some(10),
            max_length: Some(50),
        };
        assert!(!subject_config.validate_subject(commit_msg));
    }
}
