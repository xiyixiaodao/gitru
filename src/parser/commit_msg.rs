use crate::config::commit_msg_rule::{
    get_default_path_parsed_commit_msg_rule, ParsedCommitMsgRule,
};
use crate::error::commit_msg_error::CommitMsgError;
use crate::parser::header::{parse_header, ParsedHeader};
use crate::parser::preprocess_lines;

#[derive(Debug, Clone)]
pub struct ParsedCommitMessage {
    pub header: ParsedHeader,
    pub body: Option<String>,
    pub footer: Option<String>,

    pub blank_lines_before_body: usize,
    pub blank_lines_before_footer: usize,
}

/// Determine whether a line is the starting line of the footer (simple heuristic)
fn is_footer_line(line: &str, parsed_commit_msg_rule: &ParsedCommitMsgRule) -> bool {
    let trimmed = line.trim_start();
    if trimmed.is_empty() {
        return false;
    }

    let footer_keywords = match &parsed_commit_msg_rule.footer {
        Some(f) => &f.start_key_words,
        None => return false,
    };

    let lower = trimmed.to_lowercase();

    footer_keywords
        .iter()
        .any(|k| lower.starts_with(&k.to_lowercase()))
}

/// Remove leading and trailing empty lines from slice, but keep single empty lines between paragraphs
fn trim_first_and_last_empty_lines(lines: &[String]) -> Vec<String> {
    let mut v = lines.to_vec();
    while v.first().is_some_and(|l| l.trim().is_empty()) {
        v.remove(0);
    }
    while v.last().is_some_and(|l| l.trim().is_empty()) {
        v.pop();
    }
    v
}

pub fn parse_commit_msg(content: &str) -> Result<ParsedCommitMessage, CommitMsgError> {
    // 1. Preprocess
    let lines = preprocess_lines(content);

    // 2. Find the first non-empty line as the header index
    let mut idx = 0usize;
    while idx < lines.len() && lines[idx].trim().is_empty() {
        idx += 1;
    }

    // Header
    let header_raw = if idx < lines.len() {
        lines[idx].clone()
    } else {
        String::new()
    };

    // 3. Find the starting position of the footer (scan after header)
    let parsed_commit_msg_rule = &get_default_path_parsed_commit_msg_rule();
    let mut footer_start: Option<usize> = None;
    let mut i = idx + 1;
    while i < lines.len() {
        if is_footer_line(&lines[i], parsed_commit_msg_rule) {
            footer_start = Some(i);
            break;
        }
        i += 1;
    }

    // 4. Split body and footer (use index slicing to avoid frequent removals)
    let body_slice: &[String];
    let footer_slice: &[String];
    if let Some(fs) = footer_start {
        body_slice = if fs > idx + 1 {
            &lines[idx + 1..fs]
        } else {
            &[]
        };
        footer_slice = &lines[fs..];
    } else {
        body_slice = if lines.len() > idx + 1 {
            &lines[idx + 1..]
        } else {
            &[]
        };
        footer_slice = &[];
    }

    // Count blank lines before body
    let mut blank_lines_before_body = 0;
    for line in body_slice {
        if line.trim().is_empty() {
            blank_lines_before_body += 1;
        } else {
            break;
        }
    }

    // Count blank lines before footer
    let mut blank_lines_before_footer = 0;
    if !footer_slice.is_empty() {
        // footer_start is the first line of footer
        let fs = footer_start.unwrap();
        let mut j = fs - 1;

        while j > idx && lines[j].trim().is_empty() {
            blank_lines_before_footer += 1;
            j -= 1;
        }
    }

    // 5. Trim empty lines around body
    let body_lines = trim_first_and_last_empty_lines(body_slice);
    let body = if body_lines.is_empty() {
        None
    } else {
        Some(body_lines.join("\n"))
    };

    // Trim empty lines around footer
    let footer_lines = trim_first_and_last_empty_lines(footer_slice);
    let footer = if footer_lines.is_empty() {
        None
    } else {
        Some(footer_lines.join("\n"))
    };

    // 6. Parse header (using regex, compatible with Conventional Commits)
    let parsed_header = parse_header(&header_raw);

    // 7. Validate header against rules
    let parsed_header = parsed_header.map_err(CommitMsgError::Header)?;

    Ok(ParsedCommitMessage {
        header: parsed_header,
        body,
        footer,
        blank_lines_before_body,
        blank_lines_before_footer,
    })
}
