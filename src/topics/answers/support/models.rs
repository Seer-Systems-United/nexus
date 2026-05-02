#[derive(Debug, Clone)]
pub struct MappedAnswer {
    pub id: String,
    pub label: String,
    pub priority: u8,
}

#[derive(Debug, Clone)]
pub struct AggregatedAnswer {
    pub label: String,
    pub value: f32,
    pub priority: u8,
}
