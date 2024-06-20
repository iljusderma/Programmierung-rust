use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Book {
    id: Uuid,
    pub title: String,
    year: u32,
    isbn: String,
    authors: Vec<Uuid>,
}

// Implementation block for type Book
impl Book {
    pub fn new(title: String, year: u32, isbn: String, authors: Vec<Uuid>) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            year,
            isbn,
            authors,
        }
    }
}
