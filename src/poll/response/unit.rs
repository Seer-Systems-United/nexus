#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Unit {
    Other(String),
    Percent,
    Count,
}
