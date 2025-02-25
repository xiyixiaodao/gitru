// Embed config files as strings in the binary
// Separate file to maintain consistent relative paths
pub const COMMIT_MSG_HOOK_CONTENT: &str = include_str!("../../config/commit-msg-hook-template.sh");
pub const COMMIT_MSG_RULE_CONTENT: &str =
    include_str!("../../config/commit-msg-rule-template.yaml");

// Validation rule file name
pub const COMMIT_MSG_RULE_NAME: &str = ".commit-msg-rule.yaml";
