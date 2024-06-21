mod old;
mod new;
mod person;

use std::collections::HashMap;
use person::Person;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

// import old item types
use crate::library::old::book::Book as BookOld;
use crate::library::old::newspaper::Newspaper as NewspaperOld;
use crate::library::old::movie::Movie as MovieOld;

// import new item types
use crate::library::new::book::Book as Book;
pub use crate::library::new::newspaper::Newspaper as Newspaper;
use crate::library::new::movie::Movie as Movie;


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Library {
    persons: HashMap<Uuid, Person>,
    pub items: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ItemOld {
    Book(BookOld),
    Newspaper(NewspaperOld),
    Movie(MovieOld)
}


// Implementation block for type Library
impl Library {
    fn new()-> Self{
        Library{
            persons: HashMap::new(),
            items: Vec::new(),
        }
    }

    // look up the uuid of a specific person
    fn persons_uuid(&self, person: &Person) -> Option<&Uuid>{
        self.persons.iter()
        .find_map(|(key, val)| if val == person { Some(key) } else { None })
    }
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

pub fn convert(library_old: LibraryOld) -> Library{
    let mut library_new = Library::new();
    let mut persons = Vec::new();

    // extract all persons
    for item in library_old.items.iter(){
        match item{
            ItemOld::Book(b) => {
                for p in b.authors.iter(){
                    if !persons.contains(p){
                        persons.push(p.clone())
                    }
                }
            },
            ItemOld::Movie(m) => {
                if !persons.contains(&m.director){
                    persons.push(m.director.clone())
                }
            },
            _ => ()
        }
    }
    for p in persons.into_iter(){
        library_new.persons.insert(Uuid::new_v4(), p);
    }

    // save old items
    for item in library_old.items{
        match item{
            ItemOld::Book(BookOld {
                title,
                year,
                isbn,
                authors,
            }) => {
                let authors_uuids = authors.iter().map(|person| *library_new.persons_uuid(&person).unwrap())
                    .collect::<Vec<Uuid>>();
                library_new.add(Item::Book(Book::new(title, year, isbn, authors_uuids)))
                },
            ItemOld::Newspaper(NewspaperOld {
                title,
                date
            }) => library_new.add(Item::Newspaper(Newspaper::new(title, date))),
            ItemOld::Movie(MovieOld {
                title,
                year,
                director
            }) => {
                let directors_uuid = *library_new.persons_uuid(&director).unwrap();
                library_new.add(Item::Movie(Movie::new(title, year, directors_uuid)))
            },
        };
    }
    library_new
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
