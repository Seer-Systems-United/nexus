mod connection;
mod people;
mod polls;
mod responses;
mod rows;
mod schema;
mod source;

use chrono::{DateTime, Utc};

use crate::{
    database::{
        BackendTrait, person::DatabasePerson, poll::DatabasePoll, response::DatabaseResponse,
    },
    expr::{ExpressionError, ops::Filter},
};

pub use connection::{DbConnection, DbConnectionManager, DbPool, init_database};

pub struct PostgresBackend;

pub fn default_backend() -> Result<PostgresBackend, ExpressionError> {
    init_database();
    Ok(PostgresBackend)
}

impl BackendTrait for PostgresBackend {
    fn get_people(&self, filters: &[Filter]) -> Result<Vec<DatabasePerson>, ExpressionError> {
        people::get_people(filters)
    }

    fn get_polls(&self, filters: &[Filter]) -> Result<Vec<DatabasePoll>, ExpressionError> {
        polls::get_polls(filters)
    }

    fn get_responses(&self, filters: &[Filter]) -> Result<Vec<DatabaseResponse>, ExpressionError> {
        responses::get_responses(filters)
    }

    fn poll_exists_by_timestamp(&self, timestamp: DateTime<Utc>) -> Result<bool, ExpressionError> {
        polls::poll_exists_by_timestamp(timestamp)
    }
}
