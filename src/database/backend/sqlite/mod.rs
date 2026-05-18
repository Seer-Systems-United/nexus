use std::{cell::RefCell, path::Path};

use diesel::{Connection, SqliteConnection};

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

pub fn default_backend() -> Result<SqliteBackend, ExpressionError> {
    let path = std::env::var("NEXUS_SQLITE_PATH").unwrap_or_else(|_| "nexus.sqlite".to_string());
    SqliteBackend::open(path)
}

impl SqliteBackend {
    pub fn in_memory() -> Result<Self, ExpressionError> {
        Self::open(":memory:")
    }

    pub fn open(path: impl AsRef<Path>) -> Result<Self, ExpressionError> {
        let database_url = path.as_ref().to_string_lossy();
        let conn = SqliteConnection::establish(database_url.as_ref())?;
        let store = Self {
            conn: RefCell::new(conn),
        };
        store.setup_schema()?;
        Ok(store)
    }

    pub fn setup_schema(&self) -> Result<(), ExpressionError> {
        setup::setup_schema(&mut self.conn.borrow_mut())
    }

    pub fn insert_person(&self, person: &DatabasePerson) -> Result<(), ExpressionError> {
        insert::insert_person(&mut self.conn.borrow_mut(), person)
    }

    pub fn insert_poll(&self, poll: &DatabasePoll) -> Result<(), ExpressionError> {
        insert::insert_poll(&mut self.conn.borrow_mut(), poll)
    }

    pub fn insert_response(&self, response: &DatabaseResponse) -> Result<(), ExpressionError> {
        insert::insert_response(&mut self.conn.borrow_mut(), response)
    }

    pub fn insert_source(
        &self,
        id: uuid::Uuid,
        name: impl AsRef<str>,
    ) -> Result<(), ExpressionError> {
        insert::insert_source(&mut self.conn.borrow_mut(), id, name)
    }

    pub fn insert_question(
        &self,
        id: uuid::Uuid,
        poll_id: uuid::Uuid,
        text: impl AsRef<str>,
    ) -> Result<(), ExpressionError> {
        insert::insert_question(&mut self.conn.borrow_mut(), id, poll_id, text)
    }

    pub fn insert_demographic(
        &self,
        id: uuid::Uuid,
        key: impl AsRef<str>,
        demographic_type: impl AsRef<str>,
    ) -> Result<(), ExpressionError> {
        insert::insert_demographic(&mut self.conn.borrow_mut(), id, key, demographic_type)
    }

    pub fn insert_response_unit(
        &self,
        id: uuid::Uuid,
        name: impl AsRef<str>,
    ) -> Result<(), ExpressionError> {
        insert::insert_response_unit(&mut self.conn.borrow_mut(), id, name)
    }
}

impl BackendTrait for SqliteBackend {
    fn get_people(&self, filters: &[Filter]) -> Result<Vec<DatabasePerson>, ExpressionError> {
        people::get_people(&mut self.conn.borrow_mut(), filters)
    }

    fn get_polls(&self, filters: &[Filter]) -> Result<Vec<DatabasePoll>, ExpressionError> {
        polls::get_polls(&mut self.conn.borrow_mut(), filters)
    }

    fn get_questions(&self, filters: &[Filter]) -> Result<Vec<DatabaseQuestion>, ExpressionError> {
        questions::get_questions(&mut self.conn.borrow_mut(), filters)
    }

    fn get_responses(&self, filters: &[Filter]) -> Result<Vec<DatabaseResponse>, ExpressionError> {
        responses::get_responses(&mut self.conn.borrow_mut(), filters)
    }

    fn save_poll(
        &self,
        source_name: &str,
        poll: &crate::poll::Poll,
    ) -> Result<(), ExpressionError> {
        save::save_poll(&mut self.conn.borrow_mut(), source_name, poll)
    }
}
