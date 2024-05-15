use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug, Clone)]
struct Author {
    name: String,
    birthYear: usize,
}

#[derive(Deserialize, Debug, Clone)]
struct Book {
    r#type: String,
    title: String, 
    year: usize,
    author: Author
}

#[derive(Deserialize, Debug, Clone)]
enum Item {
    Book {r#type:String, title:String, year:usize, author}
}

#[derive(Deserialize, Debug, Clone)]
struct Library {
    items: Vec<Item>,
}

fn read_file() -> Result<Library, Box<dyn Error>> {
    let file = File::open("library.json")?;
    let buf_reader = BufReader::new(file);
    let library = serde_json::from_reader(buf_reader)?;
    Ok(library)
}

fn main() {
    let library = read_file().unwrap();
    println!("{:#?}", library);
}