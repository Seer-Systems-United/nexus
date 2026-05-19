use std::{fmt, marker::PhantomData};

use tracing::trace;

use crate::database::{
    BackendTrait, person::DatabasePerson, poll::DatabasePoll, question::DatabaseQuestion,
    response::DatabaseResponse,
};
use crate::expr::{
    get::GetOp,
    ops::{Filter, People, Polls, Questions, Responses, Table},
    traits::OperationTrait,
};

pub mod extensions;
pub mod get;
pub mod ops;
pub mod traits;

pub struct NexusExpression<OP: OperationTrait, TableMarker = (), Out = ()> {
    table: Option<crate::expr::ops::Table>,
    filters: Vec<Filter>,
    _op: PhantomData<OP>,
    _table: PhantomData<TableMarker>,
    _output: PhantomData<Out>,
}

impl<OP: OperationTrait, TableMarker, Out> NexusExpression<OP, TableMarker, Out> {
    pub fn new() -> Self {
        Self {
            table: None,
            filters: Vec::new(),
            _op: PhantomData,
            _table: PhantomData,
            _output: PhantomData,
        }
    }

    pub fn operation(&self) -> crate::expr::ops::Operation {
        OP::OP
    }

    pub fn table(&self) -> Option<crate::expr::ops::Table> {
        self.table
    }

    pub fn filters(&self) -> &[Filter] {
        &self.filters
    }

    pub(crate) fn select_table<NextTable, NextOut>(
        self,
        table: Table,
    ) -> NexusExpression<OP, NextTable, NextOut> {
        trace!(?table, "selecting table");
        NexusExpression {
            table: Some(table),
            filters: self.filters,
            _op: PhantomData,
            _table: PhantomData,
            _output: PhantomData,
        }
    }

    pub(crate) fn push_filter(mut self, filter: Filter) -> Self {
        trace!(?filter, "adding filter");
        self.filters.push(filter);
        self
    }
}

#[derive(Debug)]
pub enum ExpressionError {
    Connection(diesel::ConnectionError),
    Database(diesel::result::Error),
    InvalidFilter { table: Table, filter: Filter },
    InvalidDate { value: String },
    InvalidTimestamp { value: String },
    InvalidUuid { value: String },
}

impl fmt::Display for ExpressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Connection(error) => write!(f, "database connection error: {error}"),
            Self::Database(error) => write!(f, "database error: {error}"),
            Self::InvalidFilter { table, filter } => {
                write!(f, "invalid filter {filter:?} for table {table:?}")
            }
            Self::InvalidDate { value } => {
                write!(
                    f,
                    "invalid date {value:?}; expected MM-DD-YYYY or YYYY-MM-DD"
                )
            }
            Self::InvalidTimestamp { value } => {
                write!(
                    f,
                    "invalid timestamp {value:?}; expected YYYY-MM-DD HH:MM:SS"
                )
            }
            Self::InvalidUuid { value } => write!(f, "invalid UUID {value:?}"),
        }
    }
}

impl std::error::Error for ExpressionError {}

impl From<diesel::result::Error> for ExpressionError {
    fn from(error: diesel::result::Error) -> Self {
        Self::Database(error)
    }
}

impl From<diesel::ConnectionError> for ExpressionError {
    fn from(error: diesel::ConnectionError) -> Self {
        Self::Connection(error)
    }
}

impl NexusExpression<GetOp, People, DatabasePerson> {
    pub fn execute_with(
        &self,
        backend: &impl BackendTrait,
    ) -> Result<Vec<DatabasePerson>, ExpressionError> {
        backend.get_people(self.filters())
    }
}

impl NexusExpression<GetOp, Polls, DatabasePoll> {
    pub fn execute_with(
        &self,
        backend: &impl BackendTrait,
    ) -> Result<Vec<DatabasePoll>, ExpressionError> {
        backend.get_polls(self.filters())
    }
}

impl NexusExpression<GetOp, Questions, DatabaseQuestion> {
    pub fn execute_with(
        &self,
        backend: &impl BackendTrait,
    ) -> Result<Vec<DatabaseQuestion>, ExpressionError> {
        backend.get_questions(self.filters())
    }
}

impl NexusExpression<GetOp, Responses, DatabaseResponse> {
    pub fn execute_with(
        &self,
        backend: &impl BackendTrait,
    ) -> Result<Vec<DatabaseResponse>, ExpressionError> {
        backend.get_responses(self.filters())
    }
}
