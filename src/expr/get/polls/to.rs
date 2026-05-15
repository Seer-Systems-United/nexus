use diesel::{ExpressionMethods, QueryDsl};
use tracing::trace;

use crate::database::poll::DatabasePoll;
use crate::{
    expr::{
        ExpressionError, NexusExpression,
        common::{date::parse_date_end, query::PollsQuery},
        get::GetOp,
        ops::{Filter, Polls},
        traits::{FilterApplication, FilterTrait},
    },
    schema,
};

pub(crate) struct PollToFilter;

impl NexusExpression<GetOp, Polls, DatabasePoll> {
    pub fn to(self, date: impl Into<String>) -> Self {
        self.push_filter(Filter::PollTo { date: date.into() })
    }
}

impl<'a> FilterTrait<PollsQuery<'a>> for PollToFilter {
    fn apply_filter(
        query: PollsQuery<'a>,
        filter: &Filter,
        _conn: &mut diesel::PgConnection,
    ) -> Result<FilterApplication<PollsQuery<'a>>, ExpressionError> {
        match filter {
            Filter::PollTo { date } => {
                trace!(date = %date, "filtering polls to date");
                Ok(FilterApplication::Applied(query.filter(
                    schema::polls::published_timestamp.le(parse_date_end(date)?),
                )))
            }
            _ => Ok(FilterApplication::Skipped(query)),
        }
    }
}
