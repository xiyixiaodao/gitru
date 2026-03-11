use crate::constant::COMMIT_MSG_RULE_FILE_NAME;
use crate::util::path::find_repo_root;
use serde::Deserialize;

pub fn parse_commit_msg_rule(rule: &str) -> Result<ParsedCommitMsgRule, String> {
    let parsed_rule: ParsedCommitMsgRule =
        toml::from_str(rule).map_err(|e| format!("failed to parse commit msg rule: {}", e))?;

    Ok(parsed_rule)
}

pub fn get_default_path_parsed_commit_msg_rule() -> ParsedCommitMsgRule {
    let default_path = find_repo_root().join(COMMIT_MSG_RULE_FILE_NAME);

    let rule = std::fs::read_to_string(default_path).unwrap();
    parse_commit_msg_rule(rule.as_str()).unwrap()
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
    pub allowed_types: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Scope {
    pub required: bool,
    pub allowed_scopes: Vec<String>,
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_commit_msg_rule() {
        let parsed_rule = get_default_path_parsed_commit_msg_rule();
        assert_eq!(
            parsed_rule.global.as_ref().unwrap().version.as_deref(),
            Some("1.0.0")
        );

        println!("{:#?}", parsed_rule);
    }
}
