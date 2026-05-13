pub mod create;
pub mod get;
pub mod search;
pub mod update;

use chrono::NaiveDateTime;
use diesel::{
    Selectable,
    prelude::{Insertable, Queryable},
};

use crate::schema::polls;

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = polls)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DatabasePoll {
    pub id: uuid::Uuid,
    pub source_id: uuid::Uuid,
    pub published_timestamp: NaiveDateTime,
}
