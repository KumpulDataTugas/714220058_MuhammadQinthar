fn main() {
    let s = String::from("Rustacean");
    let slice = &s[0..4]; // ambil "Rust"
    println!("Slice: {}", slice);
}
