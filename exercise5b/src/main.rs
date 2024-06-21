#![allow(dead_code)]

mod library;

use std::error::Error;
use std::fs::File;
use std::io::{stdin, BufReader, BufWriter, Write};

use library::LibraryOld;

// include modules
use crate::library::{convert, Library, Item, Newspaper};

// Reading and parsing library file
// We ignore the syntax on "Box<dyn Error>>"
fn read_library_file() -> Result<LibraryOld, Box<dyn Error>> {
    let library_file = File::open("library.json")?;
    let reader = BufReader::new(library_file);
    let library = serde_json::from_reader::<BufReader<File>, LibraryOld>(reader)?;
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
    let library_old = match read_library_file() {
        Ok(library) => library,
        Err(e) => panic!("{:?}", e),
    };

    let mut library = convert(library_old);

    // Logic loop
    loop {
        println!("Give command (q - quit, l - list, d - delete, c - create, u - update, s - sort, w - write changes, ftitle - filter by title):");
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
            "s" => {
                println!("Sort all items by title.");
                library.sort_by_title();
            }
            "w" => {
                println!("Wrote changes.");
                if let Err(e) = write_library_file(&library) {
                    panic!("{:?}", e);
                }
            }
            "ftitle" => {
                println!("Enter title:");
                let title = read_string();
                library.filter_by_title(title);
            }
            other => println!("Command '{other}' is not implemented yet or invalid"),
        };
        println!();
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