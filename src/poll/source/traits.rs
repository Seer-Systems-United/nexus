use crate::poll::Poll;

pub trait PollSource {
    fn get_latest_poll() -> Poll;
}
