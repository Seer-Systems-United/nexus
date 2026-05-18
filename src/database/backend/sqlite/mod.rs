use std::{cell::RefCell, path::Path};

use diesel::{Connection, SqliteConnection};
use tracing::{debug, info, instrument};

use crate::{
    database::{
        BackendTrait, person::DatabasePerson, poll::DatabasePoll, question::DatabaseQuestion,
        response::DatabaseResponse,
    },
    expr::{ExpressionError, ops::Filter},
};

mod insert;
mod people;
mod polls;
mod question_search;
mod questions;
mod responses;
mod rows;
mod save;
mod schema;
mod setup;
mod source;
mod util;

pub struct SqliteBackend {
    conn: RefCell<SqliteConnection>,
}

pub type SqliteStore = SqliteBackend;

#[instrument]
pub fn default_backend() -> Result<SqliteBackend, ExpressionError> {
    let path = std::env::var("NEXUS_SQLITE_PATH").unwrap_or_else(|_| "nexus.sqlite".to_string());
    info!(%path, "Opening default backend");
    SqliteBackend::open(path)
}

impl SqliteBackend {
    #[instrument]
    pub fn in_memory() -> Result<Self, ExpressionError> {
        info!("Opening in-memory SQLite backend");
        Self::open(":memory:")
    }

    pub fn open(path: impl AsRef<Path>) -> Result<Self, ExpressionError> {
        let database_url = path.as_ref().to_string_lossy();
        info!(%database_url, "Establishing SQLite connection");
        let conn = SqliteConnection::establish(database_url.as_ref())?;
        let store = Self {
            conn: RefCell::new(conn),
        };
        store.setup_schema()?;
        Ok(store)
    }

    #[instrument(skip(self))]
    pub fn setup_schema(&self) -> Result<(), ExpressionError> {
        info!("Setting up schema");
        setup::setup_schema(&mut self.conn.borrow_mut())
    }

    #[instrument(skip(self, person))]
    pub fn insert_person(&self, person: &DatabasePerson) -> Result<(), ExpressionError> {
        debug!(?person, "Inserting person");
        insert::insert_person(&mut self.conn.borrow_mut(), person)
    }

    #[instrument(skip(self, poll))]
    pub fn insert_poll(&self, poll: &DatabasePoll) -> Result<(), ExpressionError> {
        debug!(?poll, "Inserting poll");
        insert::insert_poll(&mut self.conn.borrow_mut(), poll)
    }

    #[instrument(skip(self, response))]
    pub fn insert_response(&self, response: &DatabaseResponse) -> Result<(), ExpressionError> {
        debug!(?response, "Inserting response");
        insert::insert_response(&mut self.conn.borrow_mut(), response)
    }

    #[instrument(skip(self, name))]
    pub fn insert_source(
        &self,
        id: uuid::Uuid,
        name: impl AsRef<str>,
    ) -> Result<(), ExpressionError> {
        debug!(%id, name = name.as_ref(), "Inserting source");
        insert::insert_source(&mut self.conn.borrow_mut(), id, name)
    }

    #[instrument(skip(self, text))]
    pub fn insert_question(
        &self,
        id: uuid::Uuid,
        poll_id: uuid::Uuid,
        text: impl AsRef<str>,
    ) -> Result<(), ExpressionError> {
        debug!(%id, %poll_id, text = text.as_ref(), "Inserting question");
        insert::insert_question(&mut self.conn.borrow_mut(), id, poll_id, text)
    }

    #[instrument(skip(self, key, demographic_type))]
    pub fn insert_demographic(
        &self,
        id: uuid::Uuid,
        key: impl AsRef<str>,
        demographic_type: impl AsRef<str>,
    ) -> Result<(), ExpressionError> {
        debug!(%id, key = key.as_ref(), demographic_type = demographic_type.as_ref(), "Inserting demographic");
        insert::insert_demographic(&mut self.conn.borrow_mut(), id, key, demographic_type)
    }

    #[instrument(skip(self, name))]
    pub fn insert_response_unit(
        &self,
        id: uuid::Uuid,
        name: impl AsRef<str>,
    ) -> Result<(), ExpressionError> {
        debug!(%id, name = name.as_ref(), "Inserting response unit");
        insert::insert_response_unit(&mut self.conn.borrow_mut(), id, name)
    }
}

impl BackendTrait for SqliteBackend {
    #[instrument(skip(self, filters))]
    fn get_people(&self, filters: &[Filter]) -> Result<Vec<DatabasePerson>, ExpressionError> {
        debug!(filters = ?filters, "Getting people");
        people::get_people(&mut self.conn.borrow_mut(), filters)
    }

    #[instrument(skip(self, filters))]
    fn get_polls(&self, filters: &[Filter]) -> Result<Vec<DatabasePoll>, ExpressionError> {
        debug!(filters = ?filters, "Getting polls");
        polls::get_polls(&mut self.conn.borrow_mut(), filters)
    }

    #[instrument(skip(self, filters))]
    fn get_questions(&self, filters: &[Filter]) -> Result<Vec<DatabaseQuestion>, ExpressionError> {
        debug!(filters = ?filters, "Getting questions");
        questions::get_questions(&mut self.conn.borrow_mut(), filters)
    }

    #[instrument(skip(self, filters))]
    fn get_responses(&self, filters: &[Filter]) -> Result<Vec<DatabaseResponse>, ExpressionError> {
        debug!(filters = ?filters, "Getting responses");
        responses::get_responses(&mut self.conn.borrow_mut(), filters)
    }

    #[instrument(skip(self, source_name, poll))]
    fn save_poll(
        &self,
        source_name: &str,
        poll: &crate::poll::Poll,
    ) -> Result<(), ExpressionError> {
        info!(source_name, "Saving poll");
        save::save_poll(&mut self.conn.borrow_mut(), source_name, poll)
    }
}
