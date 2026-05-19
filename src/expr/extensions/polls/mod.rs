use chrono::NaiveDateTime;

use crate::database::{question::DatabaseQuestion, response::DatabaseResponse};

pub mod vec;

pub trait DatabasePollExt {
    fn get_names(&self) -> Vec<String>;
    fn get_published_timestamps(&self) -> Vec<NaiveDateTime>;
    fn get_questions(&self) -> Vec<Vec<DatabaseQuestion>>;
    fn get_responses(&self) -> Vec<Vec<DatabaseResponse>>;
}
