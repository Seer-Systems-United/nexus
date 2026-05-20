use crate::database::{
    demographic::DatabaseDemographic, poll::DatabasePoll, question::DatabaseQuestion,
};

pub mod vec;

pub trait DatabaseResponseExt {
    fn get_answers(&self) -> Vec<String>;
    fn get_demographics(&self) -> Vec<DatabaseDemographic>;
    fn get_questions(&self) -> Vec<DatabaseQuestion>;
    fn get_polls(&self) -> Vec<DatabasePoll>;
}
