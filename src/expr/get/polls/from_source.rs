use diesel::{ExpressionMethods, QueryDsl};
use tracing::trace;

use crate::database::poll::DatabasePoll;
use crate::{
    expr::{
        ExpressionError, NexusExpression,
        common::{query::PollsQuery, source::source_ids_by_name},
        get::GetOp,
        ops::{Filter, PollSourceFilter, Polls},
        traits::{FilterApplication, FilterTrait},
    },
    schema,
};

pub(crate) struct FromSourceFilter;

impl NexusExpression<GetOp, Polls, DatabasePoll> {
    pub fn from_source<S: PollSourceFilter>(self, _source: S) -> Self {
        self.push_filter(Filter::PollSource {
            source_name: S::SOURCE_NAME,
        })
    }
}

impl<'a> FilterTrait<PollsQuery<'a>> for FromSourceFilter {
    fn apply_filter(
        query: PollsQuery<'a>,
        filter: &Filter,
        conn: &mut diesel::PgConnection,
    ) -> Result<FilterApplication<PollsQuery<'a>>, ExpressionError> {
        match filter {
            Filter::PollSource { source_name } => {
                trace!(source_name = %source_name, "filtering by poll source");

                let source_ids = source_ids_by_name(source_name, conn)?;

                if source_ids.is_empty() {
                    trace!(source_name = %source_name, "no matching sources found");
                    return Ok(FilterApplication::Empty);
                }

                Ok(FilterApplication::Applied(
                    query.filter(schema::polls::source_id.eq_any(source_ids)),
                ))
            }
            _ => Ok(FilterApplication::Skipped(query)),
        }
    }
}
