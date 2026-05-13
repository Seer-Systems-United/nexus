pub mod create;
pub mod get;
pub mod search;
pub mod update;

use diesel::{
    Selectable,
    prelude::{Insertable, Queryable},
};

use crate::schema::demographics;

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = demographics)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DatabaseDemographic {
    pub id: uuid::Uuid,
    pub key: String,
    pub demographic_type: String,
    pub label: Option<String>,
    pub lower_bound: Option<i32>,
    pub upper_bound: Option<i32>,
    pub registered: Option<bool>,
}
