use crate::config::commit_msg_rule::ParsedCommitMsgRule;
use strsim::normalized_levenshtein;

/// Determine whether a line is the starting line of the footer (simple heuristic)
pub fn is_footer_line(line: &str, rule: &ParsedCommitMsgRule) -> bool {
    let trimmed = line.trim_start();
    if trimmed.is_empty() {
        return false;
    }

    let footer_cfg = match &rule.footer {
        Some(f) => f,
        None => return false,
    };

    let spell_cfg = match &footer_cfg.start_key_words_spellcheck {
        Some(s) => s,
        None => return false,
    };

    let threshold = spell_cfg.threshold;

    let (keyword, _) = match trimmed.split_once(':') {
        Some(v) => v,
        None => return false,
    };

    let keyword = keyword.trim();

    footer_cfg
        .start_key_words
        .iter()
        .any(|k| strsim::normalized_levenshtein(keyword, k) >= threshold)
}

pub fn looks_like_footer(line: &str) -> bool {
    line.contains(':')
}

#[derive(Debug)]
pub struct FooterKeywordTypo {
    /// User actually wrote the keyword (misspelled)
    pub wrong: String,

    /// The closest valid keyword (correct)
    pub correct: String,

    /// Similarity between the two keywords (0.0 ~ 1.0)
    pub similarity: f64,
}

pub fn detect_footer_keyword_typo(
    line: &str,
    threshold: f64,
    key_words: &[String],
) -> Option<FooterKeywordTypo> {
    let (keyword, _rest) = line.split_once(':')?;
    let keyword = keyword.trim();

    let mut best_match = None;
    let mut best_score = 0.0;

    for valid in key_words {
        let score = normalized_levenshtein(keyword, valid.as_str());
        if score > best_score {
            best_score = score;
            best_match = Some(valid.clone());
        }
    }

    if best_score >= threshold {
        Some(FooterKeywordTypo {
            wrong: keyword.to_string(),
            correct: best_match.unwrap(),
            similarity: best_score,
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_detect_footer_keyword_typo() {
        let line = "BAEAKING CHANGE: foo bar";
        let threshold = 0.8;

        let key_words = vec![
            "BREAKING CHANGE".to_string(),
            "Closes".to_string(),
            "Fixes".to_string(),
            "Signed-off-by".to_string(),
        ];

        assert!(detect_footer_keyword_typo(line, threshold, &key_words.clone()).is_some());
        println!(
            "{:?}",
            detect_footer_keyword_typo(line, threshold, &key_words)
        );
    }
}
