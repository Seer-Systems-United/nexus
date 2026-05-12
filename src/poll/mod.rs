use crate::poll::question::Question;

pub mod question;
pub mod response;

pub struct Poll {
    pub questions: Vec<Question>,
}
