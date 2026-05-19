use crate::database::poll::DatabasePoll;
use crate::expr::{
    NexusExpression,
    get::GetOp,
    ops::{Filter, Polls},
};

impl NexusExpression<GetOp, Polls, DatabasePoll> {
    pub fn by_ids(self, poll_ids: impl IntoIterator<Item = uuid::Uuid>) -> Self {
        self.push_filter(Filter::PollIds {
            poll_ids: poll_ids.into_iter().collect(),
        })
    }
}
