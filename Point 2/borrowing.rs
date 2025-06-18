fn print_length(s: &String) {
    println!("Length: {}", s.len());
}

fn main() {
    let s = String::from("Rustacean");
    print_length(&s); // pinjam referensi ke s
    println!("Original: {}", s); // s masih bisa digunakan
}
