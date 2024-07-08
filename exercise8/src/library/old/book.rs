use serde::{Deserialize, Serialize};
use crate::library::person::Person;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Book {
    pub(crate) title: String,
    pub(crate) year: u32,
    pub(crate) isbn: String,
    pub(crate) authors: Vec<Person>,
}

// Implementation block for type Book
impl Book {
    fn new(title: String, year: u32, isbn: String, authors: Vec<Person>) -> Self {
        Self {
            title,
            year,
            isbn,
            authors,
        }
    }
}
