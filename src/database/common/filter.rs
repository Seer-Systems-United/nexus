use tracing::warn;

use crate::expr::{
    ExpressionError,
    ops::{Filter, Table},
};

pub(crate) fn invalid_filter<T>(table: Table, filter: &Filter) -> Result<T, ExpressionError> {
    warn!(?table, ?filter, "invalid filter for table");
    Err(ExpressionError::InvalidFilter {
        table,
        filter: filter.clone(),
    })
}
