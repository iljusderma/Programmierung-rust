struct Author {
    name: String,
    year: usize,
}

struct Book {
    title: String, 
    year: usize,
    ref_author: &Author
}

fn main() {
    let author1 = Author {
        name: String::from("Dan Brown"),
        year: 1964,
    }
    let book1 = Book {
        title: String::from("The Da Vinci Code"),
        year: 2003,
        ref_author: &author1,
    }
    println!("{:#?}", book1);
}
