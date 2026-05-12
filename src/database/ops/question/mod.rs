pub mod create;
pub mod get;
pub mod search;
pub mod update;

use diesel::{Selectable, prelude::Queryable};
use diesel_full_text_search::PgTsVector;

use crate::schema::questions;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = questions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DatabaseQuestion {
    pub id: uuid::Uuid,
    pub text: String,
    pub keywords: PgTsVector,
}
