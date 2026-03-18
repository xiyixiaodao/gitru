use crate::config::commit_msg_rule::get_default_path_parsed_commit_msg_rule;
use crate::error::commit_msg_error::CommitMsgError;
use crate::error::footer_error::FooterError;
use crate::error::footer_error::FooterError::FooterKeywordTypoError;
use crate::parser::footer::{detect_footer_keyword_typo, is_footer_line, looks_like_footer};
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
    let parsed_commit_msg_rule = &get_default_path_parsed_commit_msg_rule()?;
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

    if let Some(fs) = footer_start {
        for j in (idx + 1..fs).rev() {
            if lines[j].trim().is_empty() {
                blank_lines_before_footer += 1;
            } else {
                break;
            }
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

    // 8. Validate body against rules
    if footer.is_none() && body.is_some() {
        let footer_cfg = parsed_commit_msg_rule
            .footer
            .as_ref()
            .ok_or(CommitMsgError::Footer(FooterError::MissingFooterConfig))?;

        let spell_cfg = footer_cfg
            .start_key_words_spellcheck
            .clone()
            .unwrap_or_default();

        if spell_cfg.enable {
            let threshold = spell_cfg.threshold;
            let key_words = &footer_cfg.start_key_words;

            let mut footer_block: Vec<&String> = Vec::new();
            let mut in_footer_block = false;

            for line in body.iter() {
                if looks_like_footer(line) {
                    if !in_footer_block {
                        in_footer_block = true;
                    }
                    footer_block.push(line);
                } else if in_footer_block {
                    // A body section appears → all previously detected footer‑like lines are discarded
                    footer_block.clear();
                    in_footer_block = false;
                }
            }

            // The footer_block now contains the “final contiguous block of footer‑like lines”
            for line in footer_block {
                if let Some(typo) = detect_footer_keyword_typo(line, threshold, key_words) {
                    return Err(CommitMsgError::Footer(FooterKeywordTypoError {
                        wrong: typo.wrong,
                        correct: typo.correct,
                        similarity: typo.similarity,
                        threshold,
                    }));
                }
            }
        }
    }

    Ok(ParsedCommitMessage {
        header: parsed_header,
        body,
        footer,
        blank_lines_before_body,
        blank_lines_before_footer,
    })
}
