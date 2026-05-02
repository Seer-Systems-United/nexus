use crate::topics::answers::{answer, support::models::MappedAnswer};

pub fn ballot_answer(lower: &str) -> Option<MappedAnswer> {
    if lower.contains("democratic") || lower == "democrat" {
        return Some(answer("democratic-candidate", "Democratic candidate", 3));
    }
    if lower.contains("republican") {
        return Some(answer("republican-candidate", "Republican candidate", 3));
    }
    if lower.contains("another") || lower.contains("third") {
        return Some(answer("another-candidate", "Another candidate", 3));
    }
    if lower.contains("will not") || lower.contains("do not plan") {
        return Some(answer("will-not-vote", "Will not vote", 3));
    }
    super::unsure_answer(lower)
}

pub fn direction_answer(label: &str) -> Option<MappedAnswer> {
    let lower = label.to_ascii_lowercase();
    if lower.contains("right direction") {
        Some(answer("right-direction", "Right direction", 3))
    } else if lower.contains("wrong track") {
        Some(answer("wrong-track", "Wrong track", 3))
    } else {
        super::unsure_answer(label)
    }
}
