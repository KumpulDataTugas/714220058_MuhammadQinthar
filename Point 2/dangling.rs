fn no_dangling() -> String {
    let s = String::from("No problem");
    s // Ownership dikembalikan
}

fn main() {
    let valid_string = no_dangling(); // `s` pindah ke sini
    println!("{}", valid_string);     // ✅ Aman digunakan
}
