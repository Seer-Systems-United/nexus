// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::Tsvector;

    people (id) {
        id -> Uuid,
        given_name -> Varchar,
        surname -> Varchar,
        suffix -> Nullable<Varchar>,
        prefix -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::Tsvector;

    questions (id) {
        id -> Uuid,
        text -> Varchar,
        keywords -> Tsvector,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    people,
    questions,
);
