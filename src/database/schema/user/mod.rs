//! # User schema
//!
//! Diesel schema definition for the `users` table.

use diesel::prelude::*;

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Text,
        email -> Nullable<Text>,
        account_number -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: Option<String>,
    pub account_number: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub(crate) id: uuid::Uuid,
    pub(crate) name: String,
    pub(crate) email: Option<String>,
    pub(crate) account_number: Option<String>,
    pub(crate) created_at: chrono::NaiveDateTime,
}
