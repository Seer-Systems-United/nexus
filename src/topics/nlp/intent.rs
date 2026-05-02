use crate::topics::types::TopicObservation;

pub fn intent_from_observation(observation: &TopicObservation) -> Option<String> {
    let mut answer_text = String::new();
    for demographic in &observation.demographics {
        for answer in &demographic.answers {
            answer_text.push(' ');
            answer_text.push_str(&answer.id);
            answer_text.push(' ');
            answer_text.push_str(&answer.label);
        }
    }

    intent_from_text(&super::text::normalized_search_text(&answer_text))
}

pub fn intent_from_text(text: &str) -> Option<String> {
    for (needle, intent) in SIMPLE_INTENTS {
        if text.contains(needle) {
            return Some((*intent).to_string());
        }
    }
    if text.contains("support") && text.contains("oppose")
        || text.starts_with("support for ")
        || text.contains(" support for ")
    {
        return Some("support-oppose".to_string());
    }
    if text.contains("approve") && text.contains("disapprove") || text.contains("approval") {
        return Some("approval".to_string());
    }
    if text.contains("favorable") && text.contains("unfavorable") || text.contains("favorability") {
        return Some("favorability".to_string());
    }
    if text.contains("right direction") && text.contains("wrong track") {
        return Some("direction".to_string());
    }
    if text.contains("success") && text.contains("failure") {
        return Some("success-failure".to_string());
    }
    if text.contains("which political party")
        && (text.contains("better plan")
            || text.contains("policy or approach")
            || text.contains("better job"))
    {
        return Some("party-advantage".to_string());
    }
    None
}

const SIMPLE_INTENTS: &[(&str, &str)] = &[
    ("heard about", "awareness"),
    ("have you heard", "awareness"),
    ("focus on", "attention-focus"),
    ("focused on", "attention-focus"),
    ("over the long run", "future-effect"),
    ("get better or worse", "future-effect"),
    ("will get", "future-effect"),
    (" will...", "future-effect"),
    ("will make", "future-effect"),
    ("had an impact", "impact"),
    ("impact on", "impact"),
    (" impact ", "impact"),
    ("where you live", "local-condition"),
    ("worth it", "worth-it"),
    ("not worth", "worth-it"),
    ("vote for", "vote-choice"),
    ("would you vote", "vote-choice"),
];
