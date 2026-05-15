use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use tracing::trace;

use crate::expr::ExpressionError;

pub(crate) fn parse_date_start(value: &str) -> Result<NaiveDateTime, ExpressionError> {
    trace!(value = %value, "parsing start date");
    Ok(parse_date(value)?.and_time(NaiveTime::MIN))
}

pub(crate) fn parse_date_end(value: &str) -> Result<NaiveDateTime, ExpressionError> {
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
