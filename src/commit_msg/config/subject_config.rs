use colored::Colorize;
use serde::{Deserialize, Serialize};

// Subject validation configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct SubjectConfig {
    pub require_space_after_colon: bool,
    pub min_length: u8,
    pub max_length: u8,
}

impl SubjectConfig {
    pub fn validate_subject(&self, commit_msg: &str) -> bool {
        // Extract valid first line (skip comments/empty lines)
        let first_line = commit_msg
            .lines()
            .find(|line| !line.trim_start().starts_with('#') && !line.trim().is_empty())
            .unwrap_or_else(|| {
                eprintln!("{}", "Error: Commit message cannot be empty".red());
                std::process::exit(1);
            });

        // Split type declaration and subject
        let type_and_subject: Vec<&str> = first_line.splitn(2, ':').collect();

        //Errors before the colon are handled by the corresponding section
        let subject = type_and_subject[1].trim_end();

        // Leading space check
        match (self.require_space_after_colon, subject.starts_with(' ')) {
            (false, true) => {
                eprintln!(
                    "{}",
                    "Error: Based on your custom configuration, subject should not have a prefix space ".bright_red()
                );
                return false;
            }
            (true, false) => {
                eprintln!(
                    "{}",
                    "Error: Based on your custom configuration,subject should have a leading space ".bright_red()
                );
                return false;
            }
            _ => {}
        }

        // Get a valid topic (remove first and last Spaces)
        let subject = subject.trim();

        let len = u8::try_from(subject.len()).unwrap_or_else(|_| {
            eprintln!(
                "{}",
                "Error: The subject length exceeded the 255 character limit".bright_red()
            );
            u8::MAX
        });

        if len < self.min_length || len > self.max_length {
            eprintln!(
                "{}",
                format!(
                    "Error: Subject length {} is not within the specified range:{}————{}",
                    len, self.min_length, self.max_length
                )
                .bright_red()
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
    fn test_validate_subject_space() {
        let commit_msg = "feat: Add new feature";
        let subject_config = SubjectConfig {
            require_space_after_colon: true,
            min_length: 10,
            max_length: 50,
        };
        assert!(subject_config.validate_subject(commit_msg));
    }

    #[test]
    fn test_validate_subject_space_failure() {
        let commit_msg = "feat:Add new feature";
        let subject_config = SubjectConfig {
            require_space_after_colon: true,
            min_length: 10,
            max_length: 50,
        };
        assert!(!subject_config.validate_subject(commit_msg));
    }

    #[test]
    fn test_validate_subject_space_failure2() {
        let commit_msg = "feat: Add new feature";
        let subject_config = SubjectConfig {
            require_space_after_colon: false,
            min_length: 10,
            max_length: 50,
        };
        assert!(!subject_config.validate_subject(commit_msg));
    }
    #[test]
    fn test_validate_subject_length_failure() {
        let commit_msg = "subject: This is a very long subject that exceeds the maximum length";
        let subject_config = SubjectConfig {
            require_space_after_colon: true,
            min_length: 10,
            max_length: 50,
        };
        assert!(!subject_config.validate_subject(commit_msg));
    }
}
