use crate::poll::{location::PollLocation, question::Question};

pub mod location;
pub mod question;
pub mod response;
pub mod source;

#[derive(Debug)]
pub struct Poll {
    pub questions: Vec<Question>,
    pub published_timestamp: chrono::DateTime<chrono::Utc>,
    pub location: PollLocation,
}
