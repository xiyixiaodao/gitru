use crate::constant::COMMIT_MSG_RULE_FILE_NAME;
use crate::error::commit_msg_error::{CommitMsgError, SystemError};
use crate::util::git_path::detect_current_repo;
use serde::Deserialize;

pub fn parse_commit_msg_rule(rule: &str) -> Result<ParsedCommitMsgRule, String> {
    let parsed_rule: ParsedCommitMsgRule =
        toml::from_str(rule).map_err(|e| format!("failed to parse commit msg rule: {}", e))?;

    Ok(parsed_rule)
}

pub fn get_default_path_parsed_commit_msg_rule() -> Result<ParsedCommitMsgRule, CommitMsgError> {
    let git_kind = detect_current_repo()?;
    let default_path = git_kind.config_path(COMMIT_MSG_RULE_FILE_NAME);

    let rule = std::fs::read_to_string(&default_path).map_err(|e| {
        CommitMsgError::System(SystemError::IoPath {
            path: default_path.clone(),
            source: e,
        })
    })?;

    parse_commit_msg_rule(&rule)
        .map_err(|e| CommitMsgError::System(SystemError::Parse(e.to_string())))
}

#[derive(Debug, Deserialize)]
pub struct ParsedCommitMsgRule {
    pub global: Option<GlobalRule>,
    pub header: HeaderRule,
    pub body: Option<BodyRule>,
    pub footer: Option<FooterRule>,
}

#[derive(Debug, Deserialize)]
pub struct GlobalRule {
    pub version: Option<String>,
    pub enable_validation: Option<bool>,
    pub skip_validation_words: Option<Vec<String>>,
}

impl Default for GlobalRule {
    fn default() -> Self {
        Self {
            version: Some("1.0.0".into()),
            enable_validation: Some(true),
            skip_validation_words: Some(vec![]),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct HeaderRule {
    pub r#type: Type,
    pub scope: Option<Scope>,
    pub subject: Subject,
}

#[derive(Debug, Deserialize)]
pub struct Type {
    pub allowed_types: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Scope {
    pub required: Option<bool>,
    pub allowed_scopes: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Subject {
    pub spaces_after_colon: Option<usize>,
    pub forbid_trailing_period: bool,
    pub min_length: usize,
    pub max_length: usize,
}

#[derive(Debug, Deserialize)]
pub struct BodyRule {
    pub required: bool,
    pub min_line_length: usize,
    pub max_line_length: usize,
    pub forbid_trailing_whitespace: bool,
    pub min_blank_lines_before_body: usize,
}

#[derive(Debug, Deserialize)]
pub struct FooterRule {
    pub start_key_words: Vec<String>,
    pub min_blank_lines_before_footer: usize,
    pub min_line_length: usize,
    pub max_line_length: usize,
    pub forbid_trailing_whitespace: bool,
    pub start_key_words_spellcheck: Option<StartKeyWordsSpellcheck>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StartKeyWordsSpellcheck {
    /// When enabled, if the commit contains only header + body,
    /// the body will be checked to see whether it is a misspelled footer keyword.
    /// Enabled by default.
    pub enable: bool,

    /// Similarity threshold. Default is 0.7.
    /// When the similarity exceeds this threshold, it is treated as a spelling error.
    pub threshold: f64,
}

impl Default for StartKeyWordsSpellcheck {
    fn default() -> Self {
        Self {
            enable: true,
            threshold: 0.7,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_commit_msg_rule() {
        let parsed_rule = get_default_path_parsed_commit_msg_rule().unwrap();
        assert_eq!(
            parsed_rule.global.as_ref().unwrap().version.as_deref(),
            Some("1.0.0")
        );

        println!("{:#?}", parsed_rule);
    }
}
