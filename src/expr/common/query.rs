use crate::schema;

pub(crate) type PeopleQuery<'a> = schema::people::BoxedQuery<'a, diesel::pg::Pg>;
pub(crate) type PollsQuery<'a> = schema::polls::BoxedQuery<'a, diesel::pg::Pg>;
pub(crate) type ResponsesQuery<'a> = schema::responses::BoxedQuery<'a, diesel::pg::Pg>;
