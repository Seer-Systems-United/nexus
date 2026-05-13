pub mod create;
pub mod get;
pub mod search;
pub mod update;

use diesel::{
    Selectable,
    prelude::{Insertable, Queryable},
};

use crate::schema::responses;

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = responses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DatabaseResponse {
    pub id: uuid::Uuid,
    pub question_id: uuid::Uuid,
    pub demographic_id: uuid::Uuid,
    pub unit_id: uuid::Uuid,
    pub answer: String,
    pub value: i32,
}
