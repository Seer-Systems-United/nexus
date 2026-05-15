use chrono::{DateTime, Utc};

use crate::{
    database::{person::DatabasePerson, poll::DatabasePoll, response::DatabaseResponse},
    expr::{ExpressionError, ops::Filter},
};

pub trait BackendTrait {
    fn get_people(&self, filters: &[Filter]) -> Result<Vec<DatabasePerson>, ExpressionError>;

    fn get_polls(&self, filters: &[Filter]) -> Result<Vec<DatabasePoll>, ExpressionError>;

    fn get_responses(&self, filters: &[Filter]) -> Result<Vec<DatabaseResponse>, ExpressionError>;

    fn poll_exists_by_timestamp(&self, _timestamp: DateTime<Utc>) -> Result<bool, ExpressionError> {
        Ok(false)
    }
}
