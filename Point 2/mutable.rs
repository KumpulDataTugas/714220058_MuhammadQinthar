fn add_text(s: &mut String) {
    s.push_str(" World");
}

fn main() {
    let mut s = String::from("Hello");
    add_text(&mut s); // pinjam mutable
    println!("{}", s); // Output: Hello World
}
