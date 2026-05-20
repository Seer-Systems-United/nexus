use crate::database::{
    demographic::DatabaseDemographic, poll::DatabasePoll, question::DatabaseQuestion,
    response_unit::DatabaseResponseUnit,
};

pub mod vec;

pub trait DatabaseResponseExt {
    fn get_answers(&self) -> Vec<String>;
    fn get_demographics(&self) -> Vec<DatabaseDemographic>;
    fn get_units(&self) -> Vec<DatabaseResponseUnit>;
    fn get_questions(&self) -> Vec<DatabaseQuestion>;
    fn get_polls(&self) -> Vec<DatabasePoll>;
}
