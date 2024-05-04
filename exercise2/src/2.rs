use std::io;

#[derive(Debug, Clone)]
struct Author {
    name: String,
    year: usize,
}
impl Author {
    fn new() -> Self {
        Self {
            name: String::from("Dan Brown"),
            year: 1964,
        }
    }
    
}

#[derive(Debug)]
struct Book {
    title: String, 
    year: usize,
    author: Author
}

impl Book {
    fn new(author: Author) -> Self {
        Self {
            title: String::from("The Da Vinci Code"),
            year: 2003,
            author: author,
        }
    }
    fn set_title(&mut self, title: String) {
        self.title = title
    }
    fn set_year(&mut self, year: usize) {
        self.year = year        
    }
}

fn create_books(author: Author, n: usize) -> Vec<Book> {
    let mut books = vec![];
    for _i in 1..=n {
        books.push(Book::new(author.clone()));
    }
    books
}

fn main() {
    let author1 = Author::new();
    /*
    let mut book1 = Book::new(author1.clone());
    println!("{:#?}", &book1);
    println!("-------------");
    book1.set_title(String::from("New title"));
    book1.set_year(2005);
    let book2 = Book {
        title: String::from("Deception Point"),
        year: 2001,
        author: author1.clone(),
    };
    println!("{:#?}", &book2);
    println!("-------------");
    */
    let mut books = create_books(author1.clone(), 5);
    for (i, book) in books.iter_mut().enumerate() {
        // Read input lines
        println!("Enter the title of the {}-th book:", i+1);
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        let title = input.trim().to_string();
        println!("Enter the release year of the this book:");
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        let year: usize = input.trim().parse().unwrap();
        // update books
        book.set_title(title);
        book.set_year(year);
    }
    println!("{:#?}", books);

    /*
    Frage aus der Übung:
    Kann nur mit Referenzen gearbeitet werden, um Speicherplatz zu sparen?

    In meinem Fall nein, weil es dafür erforderlich ist den Copy trait für den Author struct zu ergänzen. 
    Das ist aber nicht möglich, da das 'name' field vom nicht primitiven Typ 'String' ist. 
    Ein Pointer zu String gibt den Pointer zu einem Heap. Ein Heap kann nur kopiert werden, indem 
    neuer Speicher für den Heap allokiert wird.  
     */
}
