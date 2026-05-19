mod connection;
mod people;
mod polls;
mod questions;
mod responses;
mod rows;
mod save;
mod schema;
mod source;

use chrono::{DateTime, Utc};
use tracing::{debug, info, instrument, trace};

use crate::{
    database::{
        BackendTrait, person::DatabasePerson, poll::DatabasePoll, question::DatabaseQuestion,
        response::DatabaseResponse,
    },
    expr::{ExpressionError, ops::Filter},
};

pub use connection::{DbConnection, DbConnectionManager, DbPool, init_database};

pub struct PostgresBackend;

pub fn default_backend() -> Result<PostgresBackend, ExpressionError> {
    info!("Initializing Postgres backend");
    init_database();
    Ok(PostgresBackend)
}

impl BackendTrait for PostgresBackend {
    #[instrument(skip(self))]
    fn get_people(&self, filters: &[Filter]) -> Result<Vec<DatabasePerson>, ExpressionError> {
        trace!("Getting people with filters: {:?}", filters);
        people::get_people(filters)
    }

    #[instrument(skip(self))]
    fn get_polls(&self, filters: &[Filter]) -> Result<Vec<DatabasePoll>, ExpressionError> {
        trace!("Getting polls with filters: {:?}", filters);
        polls::get_polls(filters)
    }

    #[instrument(skip(self))]
    fn get_questions(&self, filters: &[Filter]) -> Result<Vec<DatabaseQuestion>, ExpressionError> {
        trace!("Getting questions with filters: {:?}", filters);
        questions::get_questions(filters)
    }

    #[instrument(skip(self))]
    fn get_responses(&self, filters: &[Filter]) -> Result<Vec<DatabaseResponse>, ExpressionError> {
        trace!("Getting responses with filters: {:?}", filters);
        responses::get_responses(filters)
    }

    #[instrument(skip(self, source_ids))]
    fn get_source_names_by_ids(
        &self,
        source_ids: Vec<uuid::Uuid>,
    ) -> Result<Vec<String>, ExpressionError> {
        trace!(count = source_ids.len(), "Getting source names by IDs");
        let mut conn = connection::get_connection();
        source::source_names_by_ids(&mut conn, &source_ids)
    }

    #[instrument(skip(self, poll))]
    fn save_poll(
        &self,
        source_name: &str,
        poll: &crate::poll::Poll,
    ) -> Result<(), ExpressionError> {
        debug!("Saving poll from source: {}", source_name);
        save::save_poll(source_name, poll)
    }

    #[instrument(skip(self))]
    fn poll_exists_by_timestamp(&self, timestamp: DateTime<Utc>) -> Result<bool, ExpressionError> {
        trace!("Checking if poll exists for timestamp: {}", timestamp);
        polls::poll_exists_by_timestamp(timestamp)
    }
}
