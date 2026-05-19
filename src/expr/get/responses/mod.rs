pub mod by_id;
pub mod by_ids;
pub mod from;
pub mod from_demographic;
pub mod from_question;
pub mod from_question_id;
pub mod from_source;
pub mod from_source_id;
pub mod to;

use crate::database::response::DatabaseResponse;
use crate::expr::{
    NexusExpression,
    get::GetOp,
    ops::{Responses, Table},
};

impl NexusExpression<GetOp> {
    pub fn responses(self) -> NexusExpression<GetOp, Responses, DatabaseResponse> {
        self.select_table(Table::Responses)
    }
}
