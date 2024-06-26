use serde::{Deserialize, Serialize};
use crate::read_string;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Newspaper {
    pub title: String,
    date: String,
}

// Implementation block for type Newspaper
impl Newspaper {
    pub fn new(title: String, date: String) -> Self {
        Self { title, date }
    }

    // Helper method for creating new item
    pub fn create() -> Self {
        println!("Create new 'Newspaper'");
        println!("Please enter title:");
        let title = read_string();
        println!("Please enter date published (dd.mm.yyyy or mm/yyyy):");
        let date = read_string();

        Self { title, date }
    }

    // Helper method for updating item (hard overwrite)
    pub fn update(&mut self) {
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