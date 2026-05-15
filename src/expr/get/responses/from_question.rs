use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};
use diesel_full_text_search::{TsVectorExtensions, plainto_tsquery};
use tracing::trace;

use crate::database::response::DatabaseResponse;
use crate::{
    expr::{
        ExpressionError, NexusExpression,
        common::{query::ResponsesQuery, response::apply_question_ids_filter},
        get::GetOp,
        ops::{Filter, Responses},
        traits::{FilterApplication, FilterTrait},
    },
    schema,
};

pub(crate) struct FromQuestionFilter;

impl NexusExpression<GetOp, Responses, DatabaseResponse> {
    pub fn from_question(self, question: impl Into<String>) -> Self {
        self.push_filter(Filter::ResponseQuestion {
            question: question.into(),
        })
    }
}

impl<'a> FilterTrait<ResponsesQuery<'a>> for FromQuestionFilter {
    fn apply_filter(
        query: ResponsesQuery<'a>,
        filter: &Filter,
        conn: &mut diesel::PgConnection,
    ) -> Result<FilterApplication<ResponsesQuery<'a>>, ExpressionError> {
        match filter {
            Filter::ResponseQuestion { question } => {
                trace!(question = %question, "filtering responses by question");

                let question_ids = schema::questions::table
                    .filter(
                        schema::questions::text
                            .eq(question)
                            .or(schema::questions::keywords.matches(plainto_tsquery(question))),
                    )
                    .select(schema::questions::id)
                    .load::<uuid::Uuid>(conn)?;

                Ok(apply_question_ids_filter(query, question_ids))
            }
            _ => Ok(FilterApplication::Skipped(query)),
        }
    }
}
