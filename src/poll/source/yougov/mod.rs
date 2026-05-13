pub mod api;

use crate::poll::{Poll, source::traits::PollSource};

pub struct YouGov {}

impl YouGov {
    pub const SOURCE_NAME: &'static str = "YouGov";
}

impl PollSource for YouGov {
    fn get_latest_poll() -> Poll {
        let (questions, published_timestamp) = api::latest_survey_with_timestamp();

        Poll {
            questions,
            published_timestamp,
        }
    }
}
