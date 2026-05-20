#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PollLocation {
    National,
    State { state: String },
    County { state: String, county: String },
    Other { label: String },
}
