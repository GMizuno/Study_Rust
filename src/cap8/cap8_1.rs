#[derive(Debug)]
struct Library {
    name: String,
    books: BookCollection,
}

#[derive(Clone, Debug)]
struct BookCollection(Vec<String>);

impl Library {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            books: BookCollection(Vec::new()),
        }
    }

    fn add_book(&mut self, book: &str) {
        self.books.0.push(book.to_string()); // self.books.0 pois estamos com Vec<String>
    }

    fn get_books(&self) -> BookCollection {
        self.books.clone()
    }
}

impl Iterator for BookCollection {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        match self.0.pop() {
            Some(book) => {
                println!("Accessing book: {book}");
                Some(book)
            }
            None => {
                println!("Out of books at the library!");
                None
            }
        }
    }
}

struct GivesOne;

impl Iterator for GivesOne {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        println!("Generate 1");
        Some(1)
    }
}

fn main() {
    let mut my_library = Library::new("Calgary");
    my_library.add_book("The Doom of the Darksword");
    my_library.add_book("Demian - die Geschichte einer Jugend");
    my_library.add_book("구운몽");
    my_library.add_book("吾輩は猫である");
    for item in my_library.get_books() {
        println!("{item}");
    }

    println!("\n\nEXEMPLO 2\n\n");
    let five_ones: Vec<i32> = GivesOne.into_iter().take(5).collect();
    println!("{five_ones:?}");

    // Esse codigo fica rodando de maneira continua
    // let mut iterator = GivesOne;
    // while let Some(value) = iterator.next() {
    //     println!("Value: {}", value);
    // }
}
