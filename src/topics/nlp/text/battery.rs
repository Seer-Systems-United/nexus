//! # Battery item extraction
//!
//! Extracts question items from battery-style questions.

pub fn battery_item_text(text: &str) -> Option<String> {
    if let Some((_, tail)) = text.rsplit_once('?') {
        let item = trim_battery_tail(tail);
        if is_salient_battery_item(item) {
            return Some(item.to_string());
        }
    }

    for separator in ['\u{2014}', '\u{2013}'] {
        if let Some((head, tail)) = text.rsplit_once(separator)
            && is_battery_heading(head)
        {
            let item = trim_battery_tail(tail);
            if is_salient_battery_item(item) {
                return Some(item.to_string());
            }
        }
    }

    if let Some((head, tail)) = text.rsplit_once(" - ")
        && is_battery_heading(head)
    {
        let item = trim_battery_tail(tail);
        if is_salient_battery_item(item) {
            return Some(item.to_string());
        }
    }

    None
}

fn trim_battery_tail(text: &str) -> &str {
    text.trim_matches(|ch: char| {
        ch.is_whitespace() || ch == ':' || ch == '-' || ch == '\u{2013}' || ch == '\u{2014}'
    })
}

fn is_salient_battery_item(text: &str) -> bool {
    let normalized = super::normalized_search_text(text);
    let mut salient_tokens = 0usize;

    for token in super::token_regex()
        .find_iter(&normalized)
        .map(|match_| match_.as_str())
    {
        if !crate::topics::nlp::terms::is_stopword(token) && !super::is_question_code(token) {
            salient_tokens += 1;
        }
    }

    salient_tokens >= 1 && normalized.split_whitespace().count() <= 10 && !text.contains('?')
}

fn is_battery_heading(text: &str) -> bool {
    let normalized = super::normalized_search_text(text);

    normalized.contains("approval on issues")
        || normalized.contains("favorability of")
        || normalized.contains("following issues")
        || normalized.contains("following things")
        || normalized.contains("these public figures")
        || normalized.contains("towards these public figures")
        || normalized.contains("where you live")
}
