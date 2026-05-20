use chrono::{DateTime, Utc};

use crate::{
    database::{
        demographic::DatabaseDemographic, person::DatabasePerson, poll::DatabasePoll,
        question::DatabaseQuestion, response::DatabaseResponse,
        response_unit::DatabaseResponseUnit,
    },
    expr::{ExpressionError, ops::Filter},
    poll::Poll,
};

pub trait BackendTrait {
    fn get_people(&self, filters: &[Filter]) -> Result<Vec<DatabasePerson>, ExpressionError>;

    fn get_polls(&self, filters: &[Filter]) -> Result<Vec<DatabasePoll>, ExpressionError>;

    fn get_questions(&self, filters: &[Filter]) -> Result<Vec<DatabaseQuestion>, ExpressionError>;

    fn get_responses(&self, filters: &[Filter]) -> Result<Vec<DatabaseResponse>, ExpressionError>;

    fn get_source_names_by_ids(
        &self,
        source_ids: Vec<uuid::Uuid>,
    ) -> Result<Vec<String>, ExpressionError>;

    fn get_demographics_by_ids(
        &self,
        demographic_ids: Vec<uuid::Uuid>,
    ) -> Result<Vec<DatabaseDemographic>, ExpressionError>;

    fn get_response_units_by_ids(
        &self,
        unit_ids: Vec<uuid::Uuid>,
    ) -> Result<Vec<DatabaseResponseUnit>, ExpressionError>;

    fn save_poll(&self, source_name: &str, poll: &Poll) -> Result<(), ExpressionError>;

    fn poll_exists_by_timestamp(&self, _timestamp: DateTime<Utc>) -> Result<bool, ExpressionError> {
        Ok(false)
    }
}
