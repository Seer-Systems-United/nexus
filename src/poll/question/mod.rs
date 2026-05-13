use crate::poll::response::Response;

#[derive(Debug, Clone)]
pub struct Question {
    pub text: String,
    pub responses: Vec<Response>,
}

impl Question {
    pub fn new(text: &str, responses: Vec<Response>) -> Self {
        Self {
            text: text.to_string(),
            responses,
        }
    }
}

pub fn is_non_question_text(text: &str) -> bool {
    let normalized = text.split_whitespace().collect::<Vec<_>>().join(" ");

    // Table-of-contents and methodology pages are PDF front/back matter, not poll questions.
    normalized.contains(". . .")
        || normalized.starts_with("The Economist Fieldwork")
        || normalized.contains("Fieldwork YouGov Interviewing Dates")
        || normalized.contains("Respondents were selected from YouGov")
        || normalized.contains("Questions not reported")
        || (normalized.contains("Target population")
            && normalized.contains("Sampling method")
            && normalized.contains("Margin of error"))
}
