use tracing::trace;

use crate::database::response::DatabaseResponse;
use crate::expr::{
    ExpressionError, NexusExpression,
    common::{
        poll::poll_ids_for_source_ids, query::ResponsesQuery, response::apply_poll_ids_filter,
        source::source_ids_by_name,
    },
    get::GetOp,
    ops::{Filter, Responses, SourceFilter},
    traits::{FilterApplication, FilterTrait},
};

pub(crate) struct ResponseFromSourceFilter;

impl NexusExpression<GetOp, Responses, DatabaseResponse> {
    pub fn from_source<S: SourceFilter>(self, _source: S) -> Self {
        self.push_filter(Filter::ResponseSource {
            source_name: S::SOURCE_NAME,
        })
    }
}

impl<'a> FilterTrait<ResponsesQuery<'a>> for ResponseFromSourceFilter {
    fn apply_filter(
        query: ResponsesQuery<'a>,
        filter: &Filter,
        conn: &mut diesel::PgConnection,
    ) -> Result<FilterApplication<ResponsesQuery<'a>>, ExpressionError> {
        match filter {
            Filter::ResponseSource { source_name } => {
                trace!(source_name = %source_name, "filtering responses by poll source");

                let source_ids = source_ids_by_name(source_name, conn)?;
                let poll_ids = poll_ids_for_source_ids(source_ids, conn)?;

                apply_poll_ids_filter(query, poll_ids, conn)
            }
            _ => Ok(FilterApplication::Skipped(query)),
        }
    }
}
