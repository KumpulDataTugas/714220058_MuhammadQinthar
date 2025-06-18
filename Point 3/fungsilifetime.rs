struct Book<'a> {
    title: &'a str,
}

fn read_book(book: &Book) {
    println!("Reading: {}", book.title);
}

fn main() {
    let b = Book { title: "The Rust Book" };
    read_book(&b);
}
