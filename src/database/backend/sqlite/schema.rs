diesel::table! {
    demographics (id) {
        id -> Text,
        key -> Text,
        demographic_type -> Text,
        label -> Nullable<Text>,
        lower_bound -> Nullable<Integer>,
        upper_bound -> Nullable<Integer>,
        registered -> Nullable<Bool>,
    }
}

diesel::table! {
    people (id) {
        id -> Text,
        given_name -> Text,
        surname -> Text,
        suffix -> Nullable<Text>,
        prefix -> Nullable<Text>,
    }
}

diesel::table! {
    polls (id) {
        id -> Text,
        source_id -> Text,
        published_timestamp -> Text,
    }
}

diesel::table! {
    questions (id) {
        id -> Text,
        text -> Text,
        keywords -> Text,
        poll_id -> Text,
    }
}

diesel::table! {
    response_units (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::table! {
    responses (id) {
        id -> Text,
        question_id -> Text,
        demographic_id -> Text,
        unit_id -> Text,
        answer -> Text,
        value -> Integer,
    }
}

diesel::table! {
    sources (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    demographics,
    people,
    polls,
    questions,
    response_units,
    responses,
    sources,
);
