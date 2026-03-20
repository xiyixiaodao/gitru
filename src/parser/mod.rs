// Responsible for parsing commit messages

pub mod commit_msg;
mod footer;
pub mod header;

/// Preprocess commit message: remove comment lines and empty lines
fn preprocess_lines(content: &str) -> Vec<String> {
    content
        .lines()
        .map(|line| line.trim_start_matches('\u{feff}')) // removeBOM
        .map(|line| line.trim_matches(|c: char| c.is_control())) // removeControlChars
        .filter(|line| !line.trim_start().starts_with('#')) // filterCommentLines
        .map(|s| s.to_string())
        .collect()
}

/// Get the first non-empty line (based on preprocess_lines)
pub fn get_first_non_empty_line(content: &str) -> Option<String> {
    preprocess_lines(content)
        .into_iter()
        .find(|line| !line.trim().is_empty())
}
