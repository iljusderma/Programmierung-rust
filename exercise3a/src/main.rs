use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use serde::{Serialize, Deserialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Author {
    name: String,
    #[serde(rename = "birthYear")]
    birth_year: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Book {
    title: String, 
    year: usize,
    author: Author
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Newspaper {
    title: String,
    date: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Movie {
    title: String,
    year: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
enum Item {
    Book(Book),
    Newspaper(Newspaper),
    Movie(Movie)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Library {
    items: Vec<Item>,
}

impl Library {
    fn add_item(&mut self, item: Item) -> &mut Self{
        self.items.push(item);
        self
    }
}

impl Author {
    fn new(name:String, birth_year:usize) -> Self {
        Self {
            name,
            birth_year
        }
    }
}

impl Book {
    fn new(title:String, year:usize, author:Author) -> Self {
        Self {
            title,
            year,
            author
        }
    }
}

impl Newspaper {
    fn new(title:String, date:String) -> Self {
        Self {
            title,
            date,
        }
    }
}

impl Movie {
    fn new(title:String, year:usize) -> Self {
        Self {
            title,
            year,
        }
    }
}

fn read_file() -> Result<Library, Box<dyn Error>> {
    let file = File::open("library.json")?;
    let buf_reader = BufReader::new(file);
    let library = serde_json::from_reader(buf_reader)?;
    Ok(library)
}

fn write_file(library:&Library) -> Result<(), Box<dyn Error>>{
    let file = File::create("edited_library.json").unwrap();
    let mut buf_writer = BufWriter::new(file);
    let _ = serde_json::to_writer_pretty(&mut buf_writer, library);
    Ok(())
}

fn main() {
    let mut library = read_file().unwrap();
    println!("{:#?}", library);
    
    // initialize new items
    let author = Author::new(
        "Neal Schustermann".to_string(), 
        1962);
    let book = Book::new(
        "Scythe".to_string(), 
        2016, 
        author);
    let movie = Movie::new("Nemo".to_string(), 2003);
    let newspaper = Newspaper::new("Zeit".to_string(), "19.05.2024".to_string());

    // Add new items to library
    library.add_item(Item::Book(book));
    library.add_item(Item::Newspaper(newspaper));
    library.add_item(Item::Movie(movie));
    let _ = write_file(&library);
}