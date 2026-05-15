use tracing::trace;

use crate::database::response::DatabaseResponse;
use crate::expr::{
    ExpressionError, NexusExpression,
    common::{
        date::parse_date_start, poll::poll_ids_from_date, query::ResponsesQuery,
        response::apply_poll_ids_filter,
    },
    get::GetOp,
    ops::{Filter, Responses},
    traits::{FilterApplication, FilterTrait},
};

pub(crate) struct ResponseFromFilter;

impl NexusExpression<GetOp, Responses, DatabaseResponse> {
    pub fn from(self, date: impl Into<String>) -> Self {
        self.push_filter(Filter::ResponseFrom { date: date.into() })
    }
}

impl<'a> FilterTrait<ResponsesQuery<'a>> for ResponseFromFilter {
    fn apply_filter(
        query: ResponsesQuery<'a>,
        filter: &Filter,
        conn: &mut diesel::PgConnection,
    ) -> Result<FilterApplication<ResponsesQuery<'a>>, ExpressionError> {
        match filter {
            Filter::ResponseFrom { date } => {
                trace!(date = %date, "filtering responses from date");
                let poll_ids = poll_ids_from_date(parse_date_start(date)?, conn)?;
                apply_poll_ids_filter(query, poll_ids, conn)
            }
            _ => Ok(FilterApplication::Skipped(query)),
        }
    }
}
