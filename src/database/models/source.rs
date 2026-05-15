#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatabaseSource {
    pub id: uuid::Uuid,
    pub name: String,
}
