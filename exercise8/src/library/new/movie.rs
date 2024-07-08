use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::library::old::movie::Movie as MovieOld;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Movie {
    id: Uuid,
    pub title: String,
    year: u32,
    pub director: Uuid
}

// Implementation block for type Movie 
impl Movie {
    pub fn new(title: String, year: u32, director: Uuid) -> Self {
        Self {id: Uuid::new_v4(), 
            title, 
            year , 
            director}
    }
}

impl From<MovieOld> for Movie {
    fn from(old_movie: MovieOld) -> Self{
        Self::new(old_movie.title, old_movie.year, Uuid::new_v4())
    }
}