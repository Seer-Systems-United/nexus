use crate::topics::catalog::{
    ECONOMY_APPROVAL_ID, FOREIGN_POLICY_APPROVAL_ID, GENERIC_BALLOT_ID, IMMIGRATION_APPROVAL_ID,
    IMPORTANT_PROBLEM_ID, INFLATION_APPROVAL_ID, PRESIDENTIAL_APPROVAL_ID, RIGHT_DIRECTION_ID,
    TRUMP_FAVORABILITY_ID,
};
use crate::topics::types::AnswerResult;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct MappedAnswer {
    id: String,
    label: String,
    priority: u8,
}

#[derive(Debug, Clone)]
struct AggregatedAnswer {
    label: String,
    value: f32,
    priority: u8,
}

fn slug(input: &str) -> String {
    let mut output = String::new();
    let mut last_dash = false;

    for ch in input.to_ascii_lowercase().chars() {
        if ch.is_ascii_alphanumeric() {
            output.push(ch);
            last_dash = false;
        } else if !last_dash {
            output.push('-');
            last_dash = true;
        }
    }

    output.trim_matches('-').to_string()
}

fn normalize_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn is_net_or_exact(label: &str, root: &str) -> bool {
    let lower = label.to_ascii_lowercase();
    lower == root || lower.contains("(net)") || lower.contains(" net")
}

fn approval_answer(label: &str) -> Option<MappedAnswer> {
    let normalized = normalize_text(label);
    let lower = normalized.to_ascii_lowercase();

    if lower.contains("disapprove") {
        return Some(MappedAnswer {
            id: "disapprove".to_string(),
            label: "Disapprove".to_string(),
            priority: if is_net_or_exact(&lower, "disapprove") {
                3
            } else {
                1
            },
        });
    }

    if lower.contains("approve") {
        return Some(MappedAnswer {
            id: "approve".to_string(),
            label: "Approve".to_string(),
            priority: if is_net_or_exact(&lower, "approve") {
                3
            } else {
                1
            },
        });
    }

    unsure_answer(&normalized)
}

fn favorability_answer(label: &str) -> Option<MappedAnswer> {
    let normalized = normalize_text(label);
    let lower = normalized.to_ascii_lowercase();

    if lower.contains("unfavorable") {
        return Some(MappedAnswer {
            id: "unfavorable".to_string(),
            label: "Unfavorable".to_string(),
            priority: if is_net_or_exact(&lower, "unfavorable") {
                3
            } else {
                1
            },
        });
    }

    if lower.contains("favorable") {
        return Some(MappedAnswer {
            id: "favorable".to_string(),
            label: "Favorable".to_string(),
            priority: if is_net_or_exact(&lower, "favorable") {
                3
            } else {
                1
            },
        });
    }

    if lower.contains("not heard") {
        return Some(MappedAnswer {
            id: "not-heard-of".to_string(),
            label: "Have not heard of them".to_string(),
            priority: 3,
        });
    }

    unsure_answer(&normalized)
}

fn unsure_answer(label: &str) -> Option<MappedAnswer> {
    let lower = label.to_ascii_lowercase();
    if lower.contains("don't know")
        || lower.contains("dont know")
        || lower.contains("not sure")
        || lower.contains("unsure")
    {
        return Some(MappedAnswer {
            id: "unsure".to_string(),
            label: "Unsure".to_string(),
            priority: 3,
        });
    }

    if lower.contains("skipped") {
        return Some(MappedAnswer {
            id: "skipped".to_string(),
            label: "Skipped".to_string(),
            priority: 3,
        });
    }

    None
}

fn map_answer(topic_id: &str, label: &str) -> MappedAnswer {
    let normalized = normalize_text(label);
    let lower = normalized.to_ascii_lowercase();

    if matches!(
        topic_id,
        PRESIDENTIAL_APPROVAL_ID
            | ECONOMY_APPROVAL_ID
            | INFLATION_APPROVAL_ID
            | IMMIGRATION_APPROVAL_ID
            | FOREIGN_POLICY_APPROVAL_ID
    ) && let Some(answer) = approval_answer(&normalized)
    {
        return answer;
    }

    if topic_id == TRUMP_FAVORABILITY_ID
        && let Some(answer) = favorability_answer(&normalized)
    {
        return answer;
    }

    if topic_id == RIGHT_DIRECTION_ID {
        if lower.contains("right direction") {
            return MappedAnswer {
                id: "right-direction".to_string(),
                label: "Right direction".to_string(),
                priority: 3,
            };
        }
        if lower.contains("wrong track") {
            return MappedAnswer {
                id: "wrong-track".to_string(),
                label: "Wrong track".to_string(),
                priority: 3,
            };
        }
        if let Some(answer) = unsure_answer(&normalized) {
            return answer;
        }
    }

    if topic_id == GENERIC_BALLOT_ID {
        if lower.contains("democratic") || lower == "democrat" {
            return MappedAnswer {
                id: "democratic-candidate".to_string(),
                label: "Democratic candidate".to_string(),
                priority: 3,
            };
        }
        if lower.contains("republican") {
            return MappedAnswer {
                id: "republican-candidate".to_string(),
                label: "Republican candidate".to_string(),
                priority: 3,
            };
        }
        if lower.contains("another") || lower.contains("third") {
            return MappedAnswer {
                id: "another-candidate".to_string(),
                label: "Another candidate".to_string(),
                priority: 3,
            };
        }
        if lower.contains("will not") || lower.contains("do not plan") {
            return MappedAnswer {
                id: "will-not-vote".to_string(),
                label: "Will not vote".to_string(),
                priority: 3,
            };
        }
        if let Some(answer) = unsure_answer(&normalized) {
            return answer;
        }
    }

    if lower.contains("support") && !lower.contains("oppose") {
        return MappedAnswer {
            id: "support".to_string(),
            label: "Support".to_string(),
            priority: 3,
        };
    }
    if lower.contains("oppose") {
        return MappedAnswer {
            id: "oppose".to_string(),
            label: "Oppose".to_string(),
            priority: 3,
        };
    }
    if lower.contains("not worth") {
        return MappedAnswer {
            id: "not-worth-it".to_string(),
            label: "Not worth it".to_string(),
            priority: 3,
        };
    }
    if lower.contains("worth it") {
        return MappedAnswer {
            id: "worth-it".to_string(),
            label: "Worth it".to_string(),
            priority: 3,
        };
    }
    if let Some(answer) = unsure_answer(&normalized) {
        return answer;
    }

    let id = if topic_id == IMPORTANT_PROBLEM_ID {
        format!("issue-{}", slug(&normalized))
    } else {
        slug(&normalized)
    };

    MappedAnswer {
        id,
        label: normalized,
        priority: 3,
    }
}

pub(crate) fn normalize_answers<'a>(
    topic_id: &str,
    raw_answers: impl IntoIterator<Item = (&'a str, f32)>,
) -> Vec<AnswerResult> {
    let mut answers: HashMap<String, AggregatedAnswer> = HashMap::new();

    for (label, value) in raw_answers {
        let mapped = map_answer(topic_id, label);
        answers
            .entry(mapped.id)
            .and_modify(|existing| {
                if mapped.priority > existing.priority {
                    existing.label = mapped.label.clone();
                    existing.value = value;
                    existing.priority = mapped.priority;
                } else if mapped.priority == existing.priority && mapped.priority <= 1 {
                    existing.value += value;
                }
            })
            .or_insert(AggregatedAnswer {
                label: mapped.label,
                value,
                priority: mapped.priority,
            });
    }

    let mut answers = answers
        .into_iter()
        .map(|(id, answer)| AnswerResult {
            id,
            label: answer.label,
            value: answer.value,
        })
        .collect::<Vec<_>>();
    answers.sort_by(|left, right| left.id.cmp(&right.id));
    answers
}

#[cfg(test)]
mod tests {
    use super::normalize_answers;
    use crate::topics::catalog::PRESIDENTIAL_APPROVAL_ID;

    #[test]
    fn rolls_up_approval_components() {
        let answers = normalize_answers(
            PRESIDENTIAL_APPROVAL_ID,
            [
                ("Strongly approve", 15.0),
                ("Somewhat approve", 20.0),
                ("Somewhat disapprove", 10.0),
                ("Strongly disapprove", 50.0),
            ],
        );

        let approve = answers
            .iter()
            .find(|answer| answer.id == "approve")
            .expect("approve rollup should exist");
        let disapprove = answers
            .iter()
            .find(|answer| answer.id == "disapprove")
            .expect("disapprove rollup should exist");

        assert_eq!(approve.value, 35.0);
        assert_eq!(disapprove.value, 60.0);
    }

    #[test]
    fn net_rows_override_component_rollups() {
        let answers = normalize_answers(
            PRESIDENTIAL_APPROVAL_ID,
            [
                ("Strongly approve", 15.0),
                ("Somewhat approve", 20.0),
                ("Approve (Net)", 34.0),
            ],
        );

        let approve = answers
            .iter()
            .find(|answer| answer.id == "approve")
            .expect("approve answer should exist");

        assert_eq!(approve.value, 34.0);
    }
}
