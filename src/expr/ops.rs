use crate::{expr::traits::TableTrait, poll::source::yougov::YouGov, schema};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operation {
    Get,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Table {
    Polls,
    People,
}

pub struct Polls;
pub struct People;

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
}

pub trait PollSourceFilter {
    const SOURCE_NAME: &'static str;
}

pub enum SelectedTable {
    Polls(schema::polls::table),
    People(schema::people::table),
}

impl TableTrait for Polls {
    const TABLE: Table = Table::Polls;
}

impl TableTrait for People {
    const TABLE: Table = Table::People;
}

impl PollSourceFilter for YouGov {
    const SOURCE_NAME: &'static str = YouGov::SOURCE_NAME;
}

impl Table {
    pub fn to_table(&self) -> SelectedTable {
        match self {
            Self::Polls => SelectedTable::Polls(schema::polls::table),
            Self::People => SelectedTable::People(schema::people::table),
        }
    }
}
