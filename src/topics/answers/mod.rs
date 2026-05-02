mod aggregate;
mod classify;
mod support;

use crate::topics::types::AnswerResult;
use support::models;
use support::text;

pub use aggregate::normalize_answers;

fn answer(id: &str, label: &str, priority: u8) -> models::MappedAnswer {
    models::MappedAnswer {
        id: id.to_string(),
        label: label.to_string(),
        priority,
    }
}

fn generic_answer(topic_id: &str, normalized: &str) -> models::MappedAnswer {
    let lower = normalized.to_ascii_lowercase();

    if lower.contains("support") && !lower.contains("oppose") {
        return answer("support", "Support", 3);
    }
    if lower.contains("oppose") {
        return answer("oppose", "Oppose", 3);
    }
    if lower.contains("not worth") {
        return answer("not-worth-it", "Not worth it", 3);
    }
    if lower.contains("worth it") {
        return answer("worth-it", "Worth it", 3);
    }
    if let Some(answer) = classify::unsure_answer(normalized) {
        return answer;
    }

    let id = if topic_id == crate::topics::catalog::IMPORTANT_PROBLEM_ID {
        format!("issue-{}", text::slug(normalized))
    } else {
        text::slug(normalized)
    };

    models::MappedAnswer {
        id,
        label: normalized.to_string(),
        priority: 3,
    }
}

fn to_answer_result(id: String, answer: models::AggregatedAnswer) -> AnswerResult {
    AnswerResult {
        id,
        label: answer.label,
        value: answer.value,
    }
}
