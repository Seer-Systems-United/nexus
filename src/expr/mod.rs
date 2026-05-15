use std::{fmt, marker::PhantomData};

use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use tracing::{debug, trace, warn};

use crate::database::{
    get_connection, person::DatabasePerson, poll::DatabasePoll, response::DatabaseResponse,
};
use crate::expr::{
    get::{
        GetOp,
        names::where_as::WhereAsFilter,
        polls::{from::PollFromFilter, from_source::FromSourceFilter, to::PollToFilter},
        responses::{
            from::ResponseFromFilter, from_demographic::FromDemographicFilter,
            from_question::FromQuestionFilter, from_source::ResponseFromSourceFilter,
            to::ResponseToFilter,
        },
    },
    ops::{Filter, People, Polls, Responses, Table},
    traits::{FilterApplication, FilterTrait, OperationTrait},
};
use crate::schema;

mod common;
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
    Database(diesel::result::Error),
    InvalidFilter { table: Table, filter: Filter },
    InvalidDate { value: String },
}

impl fmt::Display for ExpressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
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
        }
    }
}

impl std::error::Error for ExpressionError {}

impl From<diesel::result::Error> for ExpressionError {
    fn from(error: diesel::result::Error) -> Self {
        Self::Database(error)
    }
}

impl NexusExpression<GetOp, People, DatabasePerson> {
    pub fn execute(&self) -> Result<Vec<DatabasePerson>, ExpressionError> {
        debug!(filters = ?self.filters(), "executing Get(People) query");

        let mut conn = get_connection();
        let mut query = schema::people::table.into_boxed();

        for filter in self.filters() {
            match WhereAsFilter::apply_filter(query, filter, &mut conn)? {
                FilterApplication::Applied(next_query) => query = next_query,
                FilterApplication::Empty => return Ok(Vec::new()),
                FilterApplication::Skipped(_) => return invalid_filter(Table::People, filter),
            }
        }

        query
            .select(DatabasePerson::as_select())
            .load::<DatabasePerson>(&mut conn)
            .map_err(ExpressionError::from)
    }
}

impl NexusExpression<GetOp, Polls, DatabasePoll> {
    pub fn execute(&self) -> Result<Vec<DatabasePoll>, ExpressionError> {
        debug!(filters = ?self.filters(), "executing Get(Polls) query");

        let mut conn = get_connection();
        let mut query = schema::polls::table.into_boxed();

        for filter in self.filters() {
            let application = FromSourceFilter::apply_filter(query, filter, &mut conn)?
                .or_else(|query| PollFromFilter::apply_filter(query, filter, &mut conn))?
                .or_else(|query| PollToFilter::apply_filter(query, filter, &mut conn))?;

            match application {
                FilterApplication::Applied(next_query) => query = next_query,
                FilterApplication::Empty => return Ok(Vec::new()),
                FilterApplication::Skipped(_) => return invalid_filter(Table::Polls, filter),
            }
        }

        query
            .select(DatabasePoll::as_select())
            .load::<DatabasePoll>(&mut conn)
            .map_err(ExpressionError::from)
    }
}

impl NexusExpression<GetOp, Responses, DatabaseResponse> {
    pub fn execute(&self) -> Result<Vec<DatabaseResponse>, ExpressionError> {
        debug!(filters = ?self.filters(), "executing Get(Responses) query");

        let mut conn = get_connection();
        let mut query = schema::responses::table.into_boxed();

        for filter in self.filters() {
            let application = ResponseFromSourceFilter::apply_filter(query, filter, &mut conn)?
                .or_else(|query| ResponseFromFilter::apply_filter(query, filter, &mut conn))?
                .or_else(|query| ResponseToFilter::apply_filter(query, filter, &mut conn))?
                .or_else(|query| FromQuestionFilter::apply_filter(query, filter, &mut conn))?
                .or_else(|query| FromDemographicFilter::apply_filter(query, filter, &mut conn))?;

            match application {
                FilterApplication::Applied(next_query) => query = next_query,
                FilterApplication::Empty => return Ok(Vec::new()),
                FilterApplication::Skipped(_) => return invalid_filter(Table::Responses, filter),
            }
        }

        query
            .select(DatabaseResponse::as_select())
            .load::<DatabaseResponse>(&mut conn)
            .map_err(ExpressionError::from)
    }
}

fn invalid_filter<T>(table: Table, filter: &Filter) -> Result<T, ExpressionError> {
    warn!(?table, ?filter, "invalid filter for table");
    Err(ExpressionError::InvalidFilter {
        table,
        filter: filter.clone(),
    })
}
