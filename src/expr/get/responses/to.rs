use tracing::trace;

use crate::database::response::DatabaseResponse;
use crate::expr::{
    ExpressionError, NexusExpression,
    common::{
        date::parse_date_end, poll::poll_ids_to_date, query::ResponsesQuery,
        response::apply_poll_ids_filter,
    },
    get::GetOp,
    ops::{Filter, Responses},
    traits::{FilterApplication, FilterTrait},
};

pub(crate) struct ResponseToFilter;

impl NexusExpression<GetOp, Responses, DatabaseResponse> {
    pub fn to(self, date: impl Into<String>) -> Self {
        self.push_filter(Filter::ResponseTo { date: date.into() })
    }
}

impl<'a> FilterTrait<ResponsesQuery<'a>> for ResponseToFilter {
    fn apply_filter(
        query: ResponsesQuery<'a>,
        filter: &Filter,
        conn: &mut diesel::PgConnection,
    ) -> Result<FilterApplication<ResponsesQuery<'a>>, ExpressionError> {
        match filter {
            Filter::ResponseTo { date } => {
                trace!(date = %date, "filtering responses to date");
                let poll_ids = poll_ids_to_date(parse_date_end(date)?, conn)?;
                apply_poll_ids_filter(query, poll_ids, conn)
            }
            _ => Ok(FilterApplication::Skipped(query)),
        }
    }
}
