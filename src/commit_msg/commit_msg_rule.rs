use serde::{Deserialize, Serialize};

use super::config::{BodyConfig, FooterConfig, ScopeConfig, SubjectConfig, TypeConfig};

#[derive(Debug, Deserialize, Serialize)]
pub struct CommitMsgRule {
    pub rules: Option<RuleSet>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RuleSet {
    // type is the rust keyword,type_rule is used instead of type in yaml
    #[serde(rename = "type")]
    pub type_rule: Option<TypeConfig>,
    pub scope: Option<ScopeConfig>,
    pub subject: Option<SubjectConfig>,
    pub body: Option<BodyConfig>,
    pub footer: Option<FooterConfig>,
}
