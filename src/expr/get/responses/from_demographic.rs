use crate::{
    database::response::DatabaseResponse,
    expr::{
        NexusExpression,
        get::GetOp,
        ops::{Filter, Responses},
    },
    poll::response::demographic::{Demographic, demographic_key},
};

impl NexusExpression<GetOp, Responses, DatabaseResponse> {
    pub fn from_demographic(self, demographic: Demographic) -> Self {
        self.push_filter(Filter::ResponseDemographic {
            demographic_key: demographic_key(&demographic),
        })
    }
}
