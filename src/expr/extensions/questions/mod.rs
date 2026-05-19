use crate::database::{poll::DatabasePoll, response::DatabaseResponse};

pub mod vec;

pub trait DatabaseQuestionExt {
    fn get_questions_text(&self) -> Vec<String>;
    fn get_polls(&self) -> Vec<DatabasePoll>;
    fn get_responses(&self) -> Vec<Vec<DatabaseResponse>>;
}
