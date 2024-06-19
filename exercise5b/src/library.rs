mod book;
mod person;
mod newspaper;
mod movie;

use serde::{Serialize, Deserialize};

use crate::library::book::Book;
pub use crate::library::newspaper::Newspaper as TNewspaper;
use crate::library::movie::Movie;


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Library {
     pub items: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum Item {
    Book(Book),
    Newspaper(TNewspaper),
    Movie(Movie),
}


// Implementation block for type Library
impl Library {
    // Prints all library items with an ID (position in vector)
    pub fn list(&self) {
        let _ = self
            .items
            .iter()
            .enumerate()
            .map(|(i, curr)| println!("{:03} - {:?}", i, curr))
            .last();
    }

    // Adds an item to the library
    pub fn add(&mut self, item: Item) -> &mut Self {
        self.items.push(item);

        self
    }

    // Removes an item from the library
    pub fn del(&mut self, pos: usize) -> &mut Self {
        // Do lookup first, then call "remove()"
        if self.items.get(pos).is_some() {
            self.items.remove(pos);
        } else {
            println!("No item found to be deleted on position {pos}.");
        }

        self
    }

    // Sort library for title
    pub fn sort_by_title(&mut self){
        self.items.sort_by(|item1, item2| item1.get_title().cmp(item2.get_title()))
    }

    // Filter by title
    pub fn filter_by_title(&self, title:String){
        let _ = self
        .items
        .iter()
        .filter(|item| item.get_title() == &title)
        .map(|item| println!("{:?}", item))
        .last();
    }
}

impl Item {
    pub fn get_title(&self) -> &str{
        match self {
            Item::Book(book) => &book.title,
            Item::Newspaper(newspaper) => &newspaper.title,
            Item::Movie(movie) => &movie.title
        }
    }
}
