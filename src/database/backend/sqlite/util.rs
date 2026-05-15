use chrono::NaiveDateTime;

use crate::expr::ExpressionError;

pub(super) fn parse_uuid(value: String) -> Result<uuid::Uuid, ExpressionError> {
    uuid::Uuid::parse_str(&value).map_err(|_| ExpressionError::InvalidUuid { value })
}

pub(super) fn parse_datetime(value: &str) -> Result<NaiveDateTime, ExpressionError> {
    NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S")
        .or_else(|_| NaiveDateTime::parse_from_str(value, "%Y-%m-%dT%H:%M:%S"))
        .map_err(|_| ExpressionError::InvalidTimestamp {
            value: value.to_string(),
        })
}

pub(super) fn format_datetime(value: NaiveDateTime) -> String {
    value.format("%Y-%m-%d %H:%M:%S").to_string()
}
