// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::Tsvector;

    demographics (id) {
        id -> Uuid,
        key -> Varchar,
        demographic_type -> Varchar,
        label -> Nullable<Varchar>,
        lower_bound -> Nullable<Int4>,
        upper_bound -> Nullable<Int4>,
        registered -> Nullable<Bool>,
    }
}

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

    poll_locations (id) {
        id -> Uuid,
        poll_id -> Uuid,
        location_type -> Varchar,
        country -> Varchar,
        state -> Nullable<Varchar>,
        county -> Nullable<Varchar>,
        label -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::Tsvector;

    polls (id) {
        id -> Uuid,
        source_id -> Uuid,
        published_timestamp -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::Tsvector;

    questions (id) {
        id -> Uuid,
        text -> Varchar,
        keywords -> Tsvector,
        poll_id -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::Tsvector;

    response_units (id) {
        id -> Uuid,
        name -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::Tsvector;

    responses (id) {
        id -> Uuid,
        question_id -> Uuid,
        demographic_id -> Uuid,
        unit_id -> Uuid,
        answer -> Varchar,
        value -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::Tsvector;

    sources (id) {
        id -> Uuid,
        name -> Varchar,
    }
}

diesel::joinable!(polls -> sources (source_id));
diesel::joinable!(poll_locations -> polls (poll_id));
diesel::joinable!(questions -> polls (poll_id));
diesel::joinable!(responses -> demographics (demographic_id));
diesel::joinable!(responses -> questions (question_id));
diesel::joinable!(responses -> response_units (unit_id));

diesel::allow_tables_to_appear_in_same_query!(
    demographics,
    people,
    poll_locations,
    polls,
    questions,
    response_units,
    responses,
    sources,
);
