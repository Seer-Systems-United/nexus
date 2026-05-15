use crate::{expr::traits::TableTrait, poll::source::yougov::YouGov};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operation {
    Get,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Table {
    Polls,
    People,
    Responses,
}

pub struct Polls;
pub struct People;
pub struct Responses;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NameField {
    FirstName,
    Surname,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Filter {
    Name { field: NameField, value: String },
    PollSource { source_name: &'static str },
    PollFrom { date: String },
    PollTo { date: String },
    ResponseSource { source_name: &'static str },
    ResponseFrom { date: String },
    ResponseTo { date: String },
    ResponseQuestion { question: String },
    ResponseDemographic { demographic_key: String },
}

pub trait SourceFilter {
    const SOURCE_NAME: &'static str;
}

pub trait PollSourceFilter {
    const SOURCE_NAME: &'static str;
}

impl TableTrait for Polls {
    const TABLE: Table = Table::Polls;
}

impl TableTrait for People {
    const TABLE: Table = Table::People;
}

impl TableTrait for Responses {
    const TABLE: Table = Table::Responses;
}

impl SourceFilter for YouGov {
    const SOURCE_NAME: &'static str = YouGov::SOURCE_NAME;
}

impl PollSourceFilter for YouGov {
    const SOURCE_NAME: &'static str = YouGov::SOURCE_NAME;
}
