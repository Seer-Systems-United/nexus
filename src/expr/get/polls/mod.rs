pub mod from;
pub mod from_source;
pub mod to;

use crate::database::poll::DatabasePoll;
use crate::expr::{
    NexusExpression,
    get::GetOp,
    ops::{Polls, Table},
};

impl NexusExpression<GetOp> {
    pub fn polls(self) -> NexusExpression<GetOp, Polls, DatabasePoll> {
        self.select_table(Table::Polls)
    }
}
