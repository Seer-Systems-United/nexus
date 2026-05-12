use crate::poll::response::demographic::Demographic;

pub mod demographic;
pub mod unit;

pub struct Response {
    pub demographic: Demographic,
    pub value: String,
    pub unit: unit::Unit,
}
