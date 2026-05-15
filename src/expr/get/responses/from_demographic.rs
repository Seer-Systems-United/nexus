use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use tracing::trace;

use crate::database::{demographic::create::demographic_key, response::DatabaseResponse};
use crate::{
    expr::{
        ExpressionError, NexusExpression,
        common::query::ResponsesQuery,
        get::GetOp,
        ops::{Filter, Responses},
        traits::{FilterApplication, FilterTrait},
    },
    poll::response::demographic::Demographic,
    schema,
};

pub(crate) struct FromDemographicFilter;

impl NexusExpression<GetOp, Responses, DatabaseResponse> {
    pub fn from_demographic(self, demographic: Demographic) -> Self {
        self.push_filter(Filter::ResponseDemographic {
            demographic_key: demographic_key(&demographic),
        })
    }
}

impl<'a> FilterTrait<ResponsesQuery<'a>> for FromDemographicFilter {
    fn apply_filter(
        query: ResponsesQuery<'a>,
        filter: &Filter,
        conn: &mut diesel::PgConnection,
    ) -> Result<FilterApplication<ResponsesQuery<'a>>, ExpressionError> {
        match filter {
            Filter::ResponseDemographic { demographic_key } => {
                trace!(demographic_key = %demographic_key, "filtering responses by demographic");

                let demographic_ids = schema::demographics::table
                    .filter(schema::demographics::key.eq(demographic_key))
                    .select(schema::demographics::id)
                    .load::<uuid::Uuid>(conn)?;

                if demographic_ids.is_empty() {
                    return Ok(FilterApplication::Empty);
                }

                Ok(FilterApplication::Applied(query.filter(
                    schema::responses::demographic_id.eq_any(demographic_ids),
                )))
            }
            _ => Ok(FilterApplication::Skipped(query)),
        }
    }
}
