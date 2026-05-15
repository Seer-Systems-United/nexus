use crate::expr::ops::{Operation, Table};

pub trait OperationTrait {
    const OP: Operation;
}

pub trait TableTrait {
    const TABLE: Table;
}
