use crate::error::header_error::HeaderError;
use regex::Regex;
use std::sync::LazyLock;

#[derive(Debug, Clone)]
pub struct ParsedHeader {
    pub r#type: String,
    pub scope: Option<String>,
    // Number of spaces after colon
    pub spaces_after_colon: usize,
    pub subject: String,
    // Whether this is a breaking change, indicated by `!` before the subject
    // Example: `refactor(parser)!: change public API`
    // No need to define in config file
    pub breaking: bool,
}

pub fn parse_header(header: &str) -> Result<ParsedHeader, HeaderError> {
    // regex: type(scope)?(!)?: subject
    // type: letters/digits/_/-
    // scope: anything except ')'
    // breaking: optional '!'

    // Full-width colon (U+FF1A) is not allowed
    if header.contains('：') {
        return Err(HeaderError::FullWidthColonNotAllowed);
    }

    // Must contain half-width colon
    if !header.contains(':') {
        return Err(HeaderError::MissingColon);
    }

    // Regex to match and extract headers following Conventional Commits style,
    // capturing type, scope, breaking marker, and subject:
    // type(scope)?(!)?: subject
    // type = any character except whitespace, '(', '!', ':', supports Unicode and emoji
    static RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r"^(?P<type>[^():!\s]+)(?:\((?P<scope>[^)]+)\))?(?P<breaking>!)?:(?P<spaces>\s*)(?P<subject>.+)$"
        )
            .unwrap()
    });

    if let Some(caps) = RE.captures(header) {
        let t = caps.name("type").map(|m| m.as_str().to_string());
        let t = t.ok_or(HeaderError::EmptyType)?;

        let scope = caps.name("scope").map(|m| m.as_str().to_string());

        // Number of spaces after colon
        let spaces = caps.name("spaces").unwrap().as_str();
        // Check for full-width spaces after colon
        if spaces.contains('　') {
            return Err(HeaderError::FullWidthSpaceNotAllowed);
        }

        let actual_spaces = spaces.chars().count();

        let subject = caps.name("subject").map(|m| m.as_str().trim().to_string());
        let subject = subject.ok_or(HeaderError::EmptySubject)?;

        let breaking = caps.name("breaking").is_some();

        Ok(ParsedHeader {
            r#type: t,
            scope,
            // Number of spaces after colon
            spaces_after_colon: actual_spaces,
            subject,
            breaking,
        })
    } else {
        // Attempt to analyze and provide specific error messages to help user fix issues
        Err(analyze_header_failure(header))
    }
}

fn analyze_header_failure(header: &str) -> HeaderError {
    // Missing colon
    if !header.contains(':') {
        return HeaderError::MissingColon;
    }

    let (left, right) = header.split_once(':').unwrap();
    let left = left.trim();
    let right = right.trim();

    // Empty type
    if left.is_empty() {
        return HeaderError::EmptyType;
    }

    // Empty scope: feat():
    if left.contains("()") {
        return HeaderError::EmptyScopeWithParen {
            left: left.to_string(),
        };
    }

    // Empty subject
    if right.is_empty() {
        return HeaderError::EmptySubject;
    }

    // Scope missing right parenthesis (only check part before colon)
    if left.contains('(') && !left.contains(')') {
        return HeaderError::MissingRightParen {
            left: left.to_string(),
        };
    }

    // Scope missing left parenthesis (only check part before colon)
    if left.contains(')') && !left.contains('(') {
        return HeaderError::MissingLeftParen {
            left: left.to_string(),
        };
    }

    // Invalid breaking marker position
    // Only check part before colon, if '!' exists it must be the last character of left
    if let Some(colon_pos) = header.find(':') {
        let left = &header[..colon_pos]; // Only check type(scope)! part

        if let Some(bang_pos) = left.find('!') {
            // '!' must be at the end of type or scope
            if bang_pos != left.len() - 1 {
                return HeaderError::InvalidBreakingPosition;
            }
        }
    }

    // Type contains invalid characters (whitespace, colon, parentheses, etc.)
    // Note: only check left part, not subject
    if left.chars().any(|c| c.is_whitespace() || c == ':') {
        return HeaderError::InvalidType(left.to_string());
    }

    // Fallback
    HeaderError::InvalidHeaderFormat(header.to_string())
}
