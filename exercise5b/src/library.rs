mod old;
mod new;
mod person;

use serde::{Serialize, Deserialize};

// import old item types
use crate::library::old::book::Book as BookOld;
pub use crate::library::old::newspaper::Newspaper as NewspaperOld;
use crate::library::old::movie::Movie as MovieOld;

// import new item types
use crate::library::new::book::Book as Book;
use crate::library::new::newspaper::Newspaper as Newspaper;
use crate::library::new::movie::Movie as Movie;


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Library {
     pub items: Vec<Item>,
}

pub struct LibraryOld {
    pub items: Vec<ItemOld>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum Item {
    Book(Book),
    Newspaper(Newspaper),
    Movie(Movie),
}

pub enum ItemOld {
    BookOld(BookOld),
    NewspaperOld(NewspaperOld),
    MovieOld(MovieOld)
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

    pub fn convert(library_old: LibraryOld) -> Library{
        let mut library_new = Library{ items: Vec::new()};
        for item in library_old.items{
            let _ = match item{
                ItemOld::BookOld(BookOld {
                    title,
                    year,
                    isbn,
                    authors,
                }) => library_new.add(Item::Book(Book::new(title, year, isbn, authors))),
                ItemOld::NewspaperOld(NewspaperOld {
                    title,
                    date
                }) => library_new.add(Item::Newspaper(Newspaper::new(title, date))),
                ItemOld::MovieOld(MovieOld {
                    title,
                    year,
                    director
                }) => library_new.add(Item::Movie(Movie::new(title, year, director))),
            }
        }
        library_new
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
