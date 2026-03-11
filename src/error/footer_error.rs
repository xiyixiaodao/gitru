use thiserror::Error;

#[derive(Debug, Error)]
pub enum FooterError {
    /// Insufficient blank lines before footer
    #[error(
        "insufficient blank lines before footer, expected {min_line}, but found {current_line}"
    )]
    BlankLinesBeforeFooterNotEnough {
        min_line: usize,
        current_line: usize,
    },

    /// Invalid footer start keyword
    #[error("invalid footer start keyword, expected one of {allowed:?}, but found {actual}")]
    FooterStartKeywordInvalid {
        allowed: Vec<String>,
        actual: String,
    },

    /// Invalid footer line length
    #[error(
        "invalid length for footer line {line_number}, expected {min} ≤ length ≤ {max}, but found {actual}"
    )]
    FooterLineLengthInvalid {
        line_number: usize,
        min: usize,
        max: usize,
        actual: usize,
    },

    /// Trailing whitespace in footer line
    #[error("footer line {line_number} contains trailing whitespace")]
    FooterTrailingWhitespace { line_number: usize },
}
