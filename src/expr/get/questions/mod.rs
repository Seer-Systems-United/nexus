pub mod from;
pub mod from_question;
pub mod from_source;
pub mod to;

use crate::database::question::DatabaseQuestion;
use crate::expr::{
    NexusExpression,
    get::GetOp,
    ops::{Questions, Table},
};

impl NexusExpression<GetOp> {
    pub fn questions(self) -> NexusExpression<GetOp, Questions, DatabaseQuestion> {
        self.select_table(Table::Questions)
    }
}
