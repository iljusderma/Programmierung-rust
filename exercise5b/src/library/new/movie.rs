use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Movie {
    id: Uuid,
    pub title: String,
    year: u32,
    director: Uuid
}

// Implementation block for type Movie 
impl Movie {
    pub fn new(title: String, year: u32, director: Uuid) -> Self {
        Self {id:Uuid::new_v4(), 
            title, 
            year , 
            director}
    }
}