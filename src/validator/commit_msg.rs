use crate::config::commit_msg_rule::ParsedCommitMsgRule;
use crate::error::body_error::BodyError;
use crate::error::body_error::BodyError::{BodyLineLengthInvalid, EmptyBody, TrailingWhitespace};
use crate::error::commit_msg_error::CommitMsgError;
use crate::error::footer_error::FooterError;
use crate::error::footer_error::FooterError::{
    FooterKeywordTypoError, FooterLineLengthInvalid, FooterStartKeywordInvalid,
    FooterTrailingWhitespace,
};
use crate::error::header_error::HeaderError::{
    EmptyScope, EmptySubject, InvalidSubjectLength, NotAllowedScope, NotAllowedType,
    SpaceAfterColonNotMatch, SubjectEndsWithPeriod,
};
use crate::parser::commit_msg::ParsedCommitMessage;
use crate::parser::header::ParsedHeader;

pub fn validate_commit_msg(
    parsed_commit_msg: &ParsedCommitMessage,
    parsed_commit_msg_rule: &ParsedCommitMsgRule,
) -> Result<bool, CommitMsgError> {
    let header = &parsed_commit_msg.header;

    // 1. validate type
    let header_type = &header.r#type;
    let allowed_types = &parsed_commit_msg_rule.header.r#type.allowed_types;

    if !allowed_types.contains(header_type) {
        return Err(CommitMsgError::Header(NotAllowedType {
            r#type: header_type.clone(),
            allowed_types: allowed_types.clone(),
        }));
    }

    // 2. validate scope
    validate_scope(header, parsed_commit_msg_rule)?;

    // 3. validate subject
    validate_subject(header, parsed_commit_msg_rule)?;

    // 4. validate body
    validate_body(parsed_commit_msg, parsed_commit_msg_rule)?;

    // 5. validate footer
    validate_footer(parsed_commit_msg, parsed_commit_msg_rule)?;
    Ok(true)
}

fn validate_scope(header: &ParsedHeader, rule: &ParsedCommitMsgRule) -> Result<(), CommitMsgError> {
    let scope_cfg = match &rule.header.scope {
        Some(cfg) => cfg,
        // no scope rule → pass
        None => return Ok(()),
    };

    let scope = &header.scope;

    // scope is required → validate scope is not empty
    if scope_cfg.required && scope.is_none() {
        return Err(CommitMsgError::Header(EmptyScope));
    }

    // scope is not empty → validate scope is in allowed_scopes
    if let Some(scope) = scope {
        let allowed = &scope_cfg.allowed_scopes;
        if !allowed.is_empty() && !allowed.contains(scope) {
            return Err(CommitMsgError::Header(NotAllowedScope {
                scope: scope.to_string(),
                allowed_scopes: allowed.clone(),
            }));
        }
    }

    Ok(())
}

fn validate_subject(
    header: &ParsedHeader,
    rule: &ParsedCommitMsgRule,
) -> Result<(), CommitMsgError> {
    let subject_cfg = &rule.header.subject;
    let subject = &header.subject;

    //  validate subject is not empty
    if subject.is_empty() {
        return Err(CommitMsgError::Header(EmptySubject));
    }

    // validate subject leading spaces
    let expected_spaces = subject_cfg.spaces_after_colon.unwrap_or(1);

    if header.spaces_after_colon != expected_spaces {
        return Err(CommitMsgError::Header(SpaceAfterColonNotMatch {
            expected: expected_spaces,
            actual: header.spaces_after_colon,
        }));
    }

    // validate subject does not end with period
    if rule.header.subject.forbid_trailing_period
        && (subject.ends_with('.') || subject.ends_with('。'))
    {
        return Err(CommitMsgError::Header(SubjectEndsWithPeriod));
    }

    // validate subject length
    let subject_len = subject.chars().count();
    if subject_len < subject_cfg.min_length || subject_len > subject_cfg.max_length {
        return Err(CommitMsgError::Header(InvalidSubjectLength {
            min: subject_cfg.min_length,
            max: subject_cfg.max_length,
            actual: subject_len,
        }));
    }

    Ok(())
}

pub fn validate_body(
    parsed: &ParsedCommitMessage,
    rule: &ParsedCommitMsgRule,
) -> Result<(), CommitMsgError> {
    let Some(body_rule) = &rule.body else {
        return Ok(());
    };

    // validate body is not empty
    if body_rule.required && parsed.body.is_none() {
        return Err(CommitMsgError::Body(EmptyBody));
    }

    // body is empty and not required → pass
    let Some(body) = parsed.body.as_deref() else {
        return Ok(());
    };

    // validate blank lines before body
    if parsed.blank_lines_before_body < body_rule.min_blank_lines_before_body {
        return Err(CommitMsgError::Body(
            BodyError::BlankLinesBeforeBodyNotEnough {
                min_line: body_rule.min_blank_lines_before_body,
                current_line: parsed.blank_lines_before_body,
            },
        ));
    }

    // validate trailing whitespace
    if body_rule.forbid_trailing_whitespace {
        for (i, line) in body.lines().enumerate() {
            if line.ends_with(' ') {
                return Err(CommitMsgError::Body(TrailingWhitespace {
                    line_number: i + 1,
                }));
            }
        }
    }

    // validate line length
    for (i, line) in body.lines().enumerate() {
        let trimmed = line.trim();

        //  Allow empty lines
        if trimmed.is_empty() {
            continue;
        }

        // 3. Only validate actual body content lines
        let len = trimmed.chars().count();
        if len < body_rule.min_line_length || len > body_rule.max_line_length {
            return Err(CommitMsgError::Body(BodyLineLengthInvalid {
                line_number: i + 1,
                min: body_rule.min_line_length,
                max: body_rule.max_line_length,
                actual: len,
            }));
        }
    }

    Ok(())
}

pub fn validate_footer(
    parsed: &ParsedCommitMessage,
    rule: &ParsedCommitMsgRule,
) -> Result<(), CommitMsgError> {
    let Some(footer_rule) = &rule.footer else {
        return Ok(()); // no footer rule → pass
    };

    // footer is empty and not required → pass
    let Some(_footer) = parsed.footer.as_deref() else {
        return Ok(());
    };

    // validate blank lines before footer
    if parsed.blank_lines_before_footer < footer_rule.min_blank_lines_before_footer {
        return Err(CommitMsgError::Footer(
            FooterError::BlankLinesBeforeFooterNotEnough {
                min_line: footer_rule.min_blank_lines_before_footer,
                current_line: parsed.blank_lines_before_footer,
            },
        ));
    }

    // validate footer start keyword
    // --- Smart footer keyword validation (supports typo detection) ---
    if !footer_rule.start_key_words.is_empty() {
        let footer_text = parsed.footer.as_deref().unwrap();
        let first_line = footer_text.lines().next().unwrap_or("").trim();

        // Extract keyword before colon
        let (keyword, _) = match first_line.split_once(':') {
            Some(v) => v,
            None => {
                return Err(CommitMsgError::Footer(FooterStartKeywordInvalid {
                    allowed: footer_rule.start_key_words.clone(),
                    actual: first_line.to_string(),
                }));
            }
        };

        let keyword = keyword.trim();

        // Load spellcheck config
        let spell_cfg = footer_rule.start_key_words_spellcheck.as_ref().unwrap();
        let threshold = spell_cfg.threshold;

        // Find best match by similarity
        let best_match = footer_rule
            .start_key_words
            .iter()
            .map(|k| (k, strsim::normalized_levenshtein(keyword, k)))
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        if let Some((correct, similarity)) = best_match {
            if similarity < threshold {
                // Similarity is too low → treat as not a footer
                return Err(CommitMsgError::Footer(FooterStartKeywordInvalid {
                    actual: keyword.to_string(),
                    allowed: footer_rule.start_key_words.clone(),
                }));
            }

            if similarity < 1.0 {
                // Similarity is high enough but not a perfect match → spelling error
                return Err(CommitMsgError::Footer(FooterKeywordTypoError {
                    wrong: keyword.to_string(),
                    correct: correct.clone(),
                    similarity,
                    threshold,
                }));
            }

            // similarity == 1.0 → completely correct
        }
    }

    for (i, line) in parsed.footer.as_deref().unwrap().lines().enumerate() {
        let width = line.chars().count();

        if width < footer_rule.min_line_length || width > footer_rule.max_line_length {
            return Err(CommitMsgError::Footer(FooterLineLengthInvalid {
                line_number: i + 1,
                min: footer_rule.min_line_length,
                max: footer_rule.max_line_length,
                actual: width,
            }));
        }
    }

    // validate footer trailing whitespace
    if footer_rule.forbid_trailing_whitespace {
        for (i, line) in parsed.footer.as_deref().unwrap().lines().enumerate() {
            if line.ends_with(' ') {
                return Err(CommitMsgError::Footer(FooterTrailingWhitespace {
                    line_number: i + 1,
                }));
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;
    use crate::constant::COMMIT_MSG_RULE_TEMPLATE;
    use crate::parser::commit_msg::parse_commit_msg;

    #[test]
    fn test_validate_commit_msg() {
        let commit_msg = r#"feat: add new feature"#;
        let parsed_commit_msg = parse_commit_msg(commit_msg);
        let parsed_commit_msg_rule =
            config::commit_msg_rule::parse_commit_msg_rule(COMMIT_MSG_RULE_TEMPLATE);

        let is_valid = validate_commit_msg(
            parsed_commit_msg.as_ref().unwrap(),
            &parsed_commit_msg_rule.unwrap(),
        );
        println!("is_valid: {:?}", is_valid);
        assert!(is_valid.is_ok());
    }

    #[test]
    fn test_valid_body_and_footer() {
        let msg = "\
feat: add new API


This is body line 1
This is body line 2

BREAKING CHANGE: API changed
";

        let parsed = parse_commit_msg(msg);

        // assert_eq!(parsed.as_ref().unwrap().blank_lines_before_body, 2);
        // assert_eq!(parsed.as_ref().unwrap().blank_lines_before_footer, 1);
        println!("{:#?}", parsed);
    }
}
