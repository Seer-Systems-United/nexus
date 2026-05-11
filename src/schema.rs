// @generated automatically by Diesel CLI.

diesel::table! {
    people (id) {
        id -> Uuid,
        given_name -> Varchar,
        surname -> Varchar,
        suffix -> Nullable<Varchar>,
        prefix -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel_full_text_search::Tsvector;
    use diesel::sql_types::*;

    questions (id) {
        id -> Uuid,
        text -> Varchar,
        keywords -> Tsvector,
    }
}

diesel::joinable!(questions -> people (id));
