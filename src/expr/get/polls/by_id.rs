use crate::database::poll::DatabasePoll;
use crate::expr::{
    NexusExpression,
    get::GetOp,
    ops::{Filter, Polls},
};

impl NexusExpression<GetOp, Polls, DatabasePoll> {
    pub fn by_id(self, poll_id: uuid::Uuid) -> Self {
        self.push_filter(Filter::PollId { poll_id })
    }
}
