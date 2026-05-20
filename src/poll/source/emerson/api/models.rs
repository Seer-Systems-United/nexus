use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: u64,
    pub date: String,
    pub link: String,
    pub title: Rendered,
    pub content: Rendered,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rendered {
    pub rendered: String,
}
