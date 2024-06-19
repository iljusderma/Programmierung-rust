use serde::{Serialize, Deserialize};
use crate::library::person::Person;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Movie {
    pub title: String,
    year: u32,
    director: Person
}

// Implementation block for type Movie 
impl Movie {
    fn new(title: String, year: u32, director: Person) -> Self {
        Self { title, year , director}
    }
}