use crate::database::poll::DatabasePoll;
use crate::expr::{
    NexusExpression,
    get::GetOp,
    ops::{Filter, Polls},
};

impl NexusExpression<GetOp, Polls, DatabasePoll> {
    pub fn from(self, date: impl Into<String>) -> Self {
        self.push_filter(Filter::PollFrom { date: date.into() })
    }
}
