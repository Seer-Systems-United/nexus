use crate::poll::Poll;

pub trait PollSource {
    /// Check if there is a new poll
    fn has_new_poll() -> bool;

    // Get the latest poll
    fn get_latest_poll() -> Poll;
}
