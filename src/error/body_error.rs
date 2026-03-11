use thiserror::Error;

#[derive(Debug, Error)]
pub enum BodyError {
    #[error("body is empty")]
    EmptyBody,

    #[error("body must be preceded by {min_line} blank line(s), currently {current_line}")]
    BlankLinesBeforeBodyNotEnough {
        min_line: usize,
        current_line: usize,
    },

    #[error("line {line_number} in body contains trailing whitespace")]
    TrailingWhitespace { line_number: usize },

    #[error(
        "line {line_number} in body must have length between {min} and {max}, current length is {actual}"
    )]
    BodyLineLengthInvalid {
        line_number: usize,
        min: usize,
        max: usize,
        actual: usize,
    },
}
