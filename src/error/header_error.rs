use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum HeaderError {
    #[error(
        r###"invalid header format: {0}

Invalid commit header format. Please use the following format:
  type(scope)!: subject

Examples:
  feat(parser): add parsing feature
  fix: resolve boundary issue
  🔥optimize: improve performance

Explanation:
  - type: commit type (supports Chinese, emoji)
  - scope: optional, wrapped in parentheses
  - !: optional, indicates breaking change
  - subject: commit description
"###
    )]
    InvalidHeaderFormat(String),

    #[error("full-width colon is not allowed (use half-width colon ':')")]
    FullWidthColonNotAllowed,

    #[error("header cannot be empty")]
    EmptyHeader,

    #[error("missing colon separator, use format `type: subject`")]
    MissingColon,

    // Type validation module
    #[error("commit type cannot be empty, e.g. `feat: xxx`")]
    EmptyType,

    #[error("commit type `{0}` contains invalid characters, use letters, numbers, or emoji")]
    InvalidType(String),

    #[error(
        "commit type `{type}` is not in the allowed list, allowed types are {allowed_types:?}"
    )]
    NotAllowedType {
        r#type: String,
        allowed_types: Vec<String>,
    },

    // Scope validation module
    #[error("scope missing right parenthesis: `{left}`\nExample: `feat(parser): xxx`")]
    MissingRightParen { left: String },

    #[error("scope missing left parenthesis: `{left}`\nExample: `feat(parser): xxx`")]
    MissingLeftParen { left: String },

    #[error("scope `{0}` format is invalid, must be wrapped in parentheses, e.g. `(parser)`")]
    InvalidScope(String),

    #[error(
        "scope is required by configuration, please provide one in parentheses, e.g. `feat(parser): xxx`"
    )]
    EmptyScope,

    #[error(
        "parentheses detected but scope is empty: `{left}`\nIf scope is not needed, remove the parentheses; otherwise, provide content, e.g. `feat(parser): xxx`"
    )]
    EmptyScopeWithParen { left: String },

    #[error("scope `{scope}` is not in the allowed list, allowed scopes are {allowed_scopes:?}")]
    NotAllowedScope {
        scope: String,
        allowed_scopes: Vec<String>,
    },

    // Subject validation module
    #[error("full-width space after colon is not allowed (use half-width space ' ')")]
    FullWidthSpaceNotAllowed,

    #[error(
        "number of spaces after colon does not match, expected {expected}, found {actual}. Please add space after colon, e.g. `feat: xxx`"
    )]
    SpaceAfterColonNotMatch { expected: usize, actual: usize },

    #[error("subject ends with a period, periods are not allowed. Please remove the period")]
    SubjectEndsWithPeriod,

    #[error(
        "subject length must be between {min} and {max} characters, current length is {actual}"
    )]
    InvalidSubjectLength {
        min: usize,
        max: usize,
        actual: usize,
    },

    #[error("subject cannot be empty, please provide commit description after colon")]
    EmptySubject,

    #[error(
        "breaking marker `!` must be placed after type or scope, before colon, e.g. `feat!: xxx`"
    )]
    InvalidBreakingPosition,
}
