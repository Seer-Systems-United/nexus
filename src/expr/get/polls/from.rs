use diesel::{ExpressionMethods, QueryDsl};
use tracing::trace;

use crate::database::poll::DatabasePoll;
use crate::{
    expr::{
        ExpressionError, NexusExpression,
        common::{date::parse_date_start, query::PollsQuery},
        get::GetOp,
        ops::{Filter, Polls},
        traits::{FilterApplication, FilterTrait},
    },
    schema,
};

pub(crate) struct PollFromFilter;

impl NexusExpression<GetOp, Polls, DatabasePoll> {
    pub fn from(self, date: impl Into<String>) -> Self {
        self.push_filter(Filter::PollFrom { date: date.into() })
    }
}

impl<'a> FilterTrait<PollsQuery<'a>> for PollFromFilter {
    fn apply_filter(
        query: PollsQuery<'a>,
        filter: &Filter,
        _conn: &mut diesel::PgConnection,
    ) -> Result<FilterApplication<PollsQuery<'a>>, ExpressionError> {
        match filter {
            Filter::PollFrom { date } => {
                trace!(date = %date, "filtering polls from date");
                Ok(FilterApplication::Applied(query.filter(
                    schema::polls::published_timestamp.ge(parse_date_start(date)?),
                )))
            }
            _ => Ok(FilterApplication::Skipped(query)),
        }
    }
}
