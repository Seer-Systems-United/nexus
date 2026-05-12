use crate::database::ops::question::create::create_question_in_db;

pub struct Question {
    pub text: String,
}

impl Question {
    pub fn new(text: &str) -> Self {
        let _ = create_question_in_db(text);

        Self {
            text: text.to_string(),
        }
    }
}
