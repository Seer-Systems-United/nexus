pub mod api;

use crate::poll::{Poll, source::traits::PollSource};

pub struct Emerson;

impl Emerson {
    pub const SOURCE_NAME: &'static str = "Emerson";
}

impl PollSource for Emerson {
    fn has_new_poll() -> bool {
        api::has_new_poll()
    }

    fn get_latest_poll() -> Poll {
        api::latest_poll()
    }
}
