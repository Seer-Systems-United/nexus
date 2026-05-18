pub mod names;
pub mod polls;
pub mod questions;
pub mod responses;

use crate::expr::{NexusExpression, ops::Operation, traits::OperationTrait};

pub struct GetOp;
impl OperationTrait for GetOp {
    const OP: Operation = Operation::Get;
}

pub fn get() -> NexusExpression<GetOp> {
    NexusExpression::new()
}
