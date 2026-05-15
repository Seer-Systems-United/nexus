use crate::database::poll::DatabasePoll;
use crate::expr::{
    NexusExpression,
    get::GetOp,
    ops::{Filter, PollSourceFilter, Polls},
};

impl NexusExpression<GetOp, Polls, DatabasePoll> {
    pub fn from_source<S: PollSourceFilter>(self, _source: S) -> Self {
        self.push_filter(Filter::PollSource {
            source_name: S::SOURCE_NAME,
        })
    }
}
