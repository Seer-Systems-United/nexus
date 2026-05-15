use std::{fmt, marker::PhantomData};

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use tracing::{debug, trace, warn};

use crate::database::{get_connection, person::DatabasePerson, poll::DatabasePoll};
use crate::expr::{
    get::GetOp,
    ops::{Filter, Table},
    ops::{NameField, People, Polls},
    traits::OperationTrait,
};
use crate::schema;

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
            match filter {
                Filter::Name {
                    field: NameField::FirstName,
                    value,
                } => {
                    trace!(value = %value, "filtering by given_name");
                    query = query.filter(schema::people::given_name.eq(value.as_str()));
                }
                Filter::Name {
                    field: NameField::Surname,
                    value,
                } => {
                    trace!(value = %value, "filtering by surname");
                    query = query.filter(schema::people::surname.eq(value.as_str()));
                }
                filter => {
                    warn!(?filter, "invalid filter for People");
                    return Err(ExpressionError::InvalidFilter {
                        table: Table::People,
                        filter: filter.clone(),
                    });
                }
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
            match filter {
                Filter::PollSource { source_name } => {
                    trace!(source_name = %source_name, "filtering by poll source");

                    let source_ids = schema::sources::table
                        .filter(schema::sources::name.eq(*source_name))
                        .select(schema::sources::id)
                        .load::<uuid::Uuid>(&mut conn)?;

                    if source_ids.is_empty() {
                        trace!(source_name = %source_name, "no matching sources found");
                        return Ok(Vec::new());
                    }

                    query = query.filter(schema::polls::source_id.eq_any(source_ids));
                }
                Filter::PollFrom { date } => {
                    trace!(date = %date, "filtering polls from date");
                    query = query
                        .filter(schema::polls::published_timestamp.ge(parse_date_start(date)?));
                }
                Filter::PollTo { date } => {
                    trace!(date = %date, "filtering polls to date");
                    query =
                        query.filter(schema::polls::published_timestamp.le(parse_date_end(date)?));
                }
                filter => {
                    warn!(?filter, "invalid filter for Polls");
                    return Err(ExpressionError::InvalidFilter {
                        table: Table::Polls,
                        filter: filter.clone(),
                    });
                }
            }
        }

        query
            .select(DatabasePoll::as_select())
            .load::<DatabasePoll>(&mut conn)
            .map_err(ExpressionError::from)
    }
}

fn parse_date_start(value: &str) -> Result<NaiveDateTime, ExpressionError> {
    trace!(value = %value, "parsing start date");
    Ok(parse_date(value)?.and_time(NaiveTime::MIN))
}

fn parse_date_end(value: &str) -> Result<NaiveDateTime, ExpressionError> {
    trace!(value = %value, "parsing end date");
    Ok(parse_date(value)?
        .and_time(NaiveTime::from_hms_opt(23, 59, 59).expect("23:59:59 is a valid time")))
}

fn parse_date(value: &str) -> Result<NaiveDate, ExpressionError> {
    trace!(value = %value, "parsing date");
    NaiveDate::parse_from_str(value, "%m-%d-%Y")
        .or_else(|_| NaiveDate::parse_from_str(value, "%Y-%m-%d"))
        .map_err(|_| ExpressionError::InvalidDate {
            value: value.to_string(),
        })
}
