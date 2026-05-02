mod battery;
mod clean;
mod regexes;

pub use clean::{clean_question_text, normalized_question_key};
pub use regexes::{is_question_code, token_regex};

use crate::topics::types::TopicObservation;

pub fn normalized_search_text(text: &str) -> String {
    let lower = text
        .to_ascii_lowercase()
        .replace("u.s.", " us ")
        .replace("u. s.", " us ")
        .replace("united states", " us ")
        .replace("don't", " dont ")
        .replace("don’t", " dont ");

    lower.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn focus_question_text(text: &str) -> String {
    clean::clean_question_text_generic(text)
}

pub fn battery_item_text(text: &str) -> Option<String> {
    battery::battery_item_text(text)
}

pub fn observation_question_text(observation: &TopicObservation) -> String {
    let title = clean::clean_question_text_generic(&observation.question_title);
    let prompt = clean::clean_question_text_generic(&observation.prompt);

    if let Some(item) = battery_item_text(&title)
        && !prompt.is_empty()
        && !title.eq_ignore_ascii_case(&prompt)
    {
        return format!("{prompt} {item}");
    }

    if prompt.is_empty() || title.eq_ignore_ascii_case(&prompt) || title.contains(&prompt) {
        title
    } else if title.is_empty() {
        prompt
    } else {
        format!("{title}: {prompt}")
    }
}
