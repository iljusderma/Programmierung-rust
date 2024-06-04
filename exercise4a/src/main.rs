#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::{stdin, BufReader, BufWriter, Write};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Library {
    items: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
enum Item {
    Book(Book),
    Newspaper(Newspaper),
    Movie(Movie),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Author {
    name: String,
    #[serde(rename = "birthYear")]
    b_year: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Book {
    title: String,
    year: u32,
    author: Author,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Newspaper {
    title: String,
    date: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Movie {
    title: String,
    year: u32,
}

// Implementation block for type Library
impl Library {
    // Prints all library items with an ID (position in vector)
    fn list(&self) {
        let _ = self
            .items
            .iter()
            .enumerate()
            .map(|(i, curr)| println!("{:03} - {:?}", i, curr))
            .last();
        // for (i, curr) in self.items.iter().enumerate() {
        //     println!("{:03} - {:?}", i, curr);
        // }
    }

    // Adds an item to the library
    fn add(&mut self, item: Item) -> &mut Self {
        self.items.push(item);

        self
    }

    // Removes an item from the library
    fn del(&mut self, pos: usize) -> &mut Self {
        // Do lookup first, then call "remove()"
        if self.items.get(pos).is_some() {
            self.items.remove(pos);
        } else {
            println!("No item found to be deleted on position {pos}.");
        }

        self
    }
}

// Implementation block for type Author
impl Author {
    fn new(name: String, b_year: u32) -> Self {
        Self { name, b_year }
    }
}

// Implementation block for type Book
impl Book {
    fn new(title: String, year: u32, author: Author) -> Self {
        Self {
            title,
            year,
            author,
        }
    }
}

// Implementation block for type Newspaper
impl Newspaper {
    fn new(title: String, date: String) -> Self {
        Self { title, date }
    }

    // Helper method for creating new item
    fn create() -> Self {
        println!("Create new 'Newspaper'");
        println!("Please enter title:");
        let title = read_string();
        println!("Please enter date published (dd.mm.yyyy or mm/yyyy):");
        let date = read_string();

        Self { title, date }
    }

    // Helper method for updating item (hard overwrite)
    fn update(&mut self) {
        println!("Update 'Newspaper' (just hit [Enter] if not changed)");
        println!("Enter new title (is: {:?}):", self.title);
        let title = read_string();
        println!("Enter new date (is: {:?}):", self.date);
        let date = read_string();

        if !title.is_empty() {
            self.title = title;
        }
        if !date.is_empty() {
            self.date = date;
        }
    }
}

// Implementation block for type Movie
impl Movie {
    fn new(title: String, year: u32) -> Self {
        Self { title, year }
    }
}

// Reading and parsing library file
// We ignore the syntax on "Box<dyn Error>>"
fn read_library_file() -> Result<Library, Box<dyn Error>> {
    let library_file = File::open("library.json")?;
    let reader = BufReader::new(library_file);
    let library = serde_json::from_reader::<BufReader<File>, Library>(reader)?;

    Ok(library)
}

// Writing back to file
// We ignore the syntax on "Box<dyn Error>>"
fn write_library_file(library: &Library) -> Result<(), Box<dyn Error>> {
    let library_file = File::create("library.json")?;
    let mut writer = BufWriter::new(library_file);
    serde_json::to_writer_pretty(&mut writer, library)?;
    writer.flush()?;

    Ok(())
}

fn main() {
    // First step: Load file into custom type "Library"
    let mut library = match read_library_file() {
        Ok(library) => library,
        Err(e) => panic!("{:?}", e),
    };

    // Logic loop
    loop {
        println!("Give command (q - quit, l - list, d - delete, c - create, u - update):");
        let command = read_string();
        match command.as_str() {
            "q" => break,
            "l" => {
                println!("List all items in library with ID:\n");
                library.list()
            }
            "c" => {
                println!("Select subtype for new item (n - newspaper):");
                let subtype = read_string();
                match subtype.as_str() {
                    "n" => {
                        library.add(Item::Newspaper(Newspaper::create()));
                    }
                    other => {
                        println!("Create not implemented for subtype '{other}' yet or invalid")
                    }
                }
            }
            "d" => {
                println!("Please enter ID of item to be deleted from library:");
                let _ = library.del(read_uint() as usize);
            }
            "u" => {
                println!("Select item by ID:");
                let item_id = read_uint() as usize;
                if let Some(item) = library.items.get_mut(item_id) {
                    match item {
                        Item::Newspaper(np) => {
                            np.update();
                        }
                        _ => println!("Update for current item either not implemented or invalid"),
                    }
                }
            }
            other => println!("Command '{other}' is not implemented yet or invalid"),
        };
        println!();
    }

    // Final step: Write all changes back to file
    if let Err(e) = write_library_file(&library) {
        panic!("{:?}", e);
    }
}

// Helper: Reads stdio input and returns String
fn read_string() -> String {
    let mut buffer = String::new();
    let _ = stdin().read_line(&mut buffer);
    buffer.trim().to_string()
}

// Helper: Reads stdio input and returns u32
fn read_uint() -> u32 {
    let mut buffer = String::new();
    let _ = stdin().read_line(&mut buffer);
    buffer.trim().parse::<u32>().unwrap()
}