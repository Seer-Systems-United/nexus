use super::support::models::AggregatedAnswer;
use crate::topics::types::AnswerResult;
use std::collections::HashMap;

pub fn normalize_answers<'a>(
    topic_id: &str,
    raw_answers: impl IntoIterator<Item = (&'a str, f32)>,
) -> Vec<AnswerResult> {
    let mut answers: HashMap<String, AggregatedAnswer> = HashMap::new();

    for (label, value) in raw_answers {
        let mapped = super::classify::map_answer(topic_id, label);
        answers
            .entry(mapped.id)
            .and_modify(|existing| merge_answer(existing, &mapped.label, value, mapped.priority))
            .or_insert(AggregatedAnswer {
                label: mapped.label,
                value,
                priority: mapped.priority,
            });
    }

    let mut answers = answers
        .into_iter()
        .map(|(id, answer)| super::to_answer_result(id, answer))
        .collect::<Vec<_>>();
    answers.sort_by(|left, right| left.id.cmp(&right.id));
    answers
}

fn merge_answer(existing: &mut AggregatedAnswer, label: &str, value: f32, priority: u8) {
    if priority > existing.priority {
        existing.label = label.to_string();
        existing.value = value;
        existing.priority = priority;
    } else if priority == existing.priority && priority <= 1 {
        existing.value += value;
    }
}
