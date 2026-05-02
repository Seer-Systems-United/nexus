//! # Regex patterns
//!
//! Compiled regex patterns for text processing.
//! Uses lazy initialization for efficiency.

use regex::Regex;
use std::sync::OnceLock;

pub fn token_regex() -> &'static Regex {
    static TOKEN_REGEX: OnceLock<Regex> = OnceLock::new();
    TOKEN_REGEX.get_or_init(|| Regex::new(r"[a-z][a-z0-9]+").expect("valid token regex"))
}

pub fn is_question_code(token: &str) -> bool {
    let has_digit = token.chars().any(|ch| ch.is_ascii_digit());
    let has_alpha = token.chars().any(|ch| ch.is_ascii_alphabetic());
    has_digit && has_alpha && (token.len() <= 16 || token.starts_with("tm"))
}

pub fn question_code_regex() -> &'static Regex {
    static QUESTION_CODE_REGEX: OnceLock<Regex> = OnceLock::new();
    QUESTION_CODE_REGEX.get_or_init(|| {
        Regex::new(r"(?i)(?:^|[:\s])(?:[a-z][a-z0-9_-]{0,24}|\d{1,3}[a-z]?)\.\s+")
            .expect("valid question code regex")
    })
}

pub fn leading_question_code_regex() -> &'static Regex {
    static LEADING_QUESTION_CODE_REGEX: OnceLock<Regex> = OnceLock::new();
    LEADING_QUESTION_CODE_REGEX.get_or_init(|| {
        Regex::new(r"(?i)^(?:[a-z][a-z0-9_-]{0,24}|\d{1,3}[a-z]?)\.\s+")
            .expect("valid leading question code regex")
    })
}

pub fn ipsos_metadata_prefix_regex() -> &'static Regex {
    static IPSOS_METADATA_PREFIX_REGEX: OnceLock<Regex> = OnceLock::new();
    IPSOS_METADATA_PREFIX_REGEX.get_or_init(|| {
        Regex::new(r"(?i)^\d{4}-\d{2}-\d{2}:\s+[^:]{3,240}:\s+(.+)$")
            .expect("valid Ipsos metadata prefix regex")
    })
}

pub fn yougov_metadata_prefix_regex() -> &'static Regex {
    static YOUGOV_METADATA_PREFIX_REGEX: OnceLock<Regex> = OnceLock::new();
    YOUGOV_METADATA_PREFIX_REGEX.get_or_init(|| {
        Regex::new(
            r"(?i)^[a-z]+ \d{1,2}\s*-\s*\d{1,2},\s*\d{4}\s*-\s*[\d,]+\s+u\.s\. adult citizens:\s+(.+)$",
        )
        .expect("valid YouGov metadata prefix regex")
    })
}
