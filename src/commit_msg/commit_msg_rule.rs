use serde::{Deserialize, Serialize};

use super::config::{BodyConfig, FooterConfig, ScopeConfig, SubjectConfig, TypeConfig};

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
