#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatabaseResponseUnit {
    pub id: uuid::Uuid,
    pub name: String,
}
