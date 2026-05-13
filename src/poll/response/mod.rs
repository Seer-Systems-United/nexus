use std::sync::Arc;

use crate::poll::response::demographic::Demographic;

pub mod demographic;
pub mod unit;

#[derive(Debug, Clone)]
pub struct Response {
    pub demographic: Demographic,
    pub answer: Arc<str>,
    pub value: u16,
    pub unit: unit::Unit,
}
