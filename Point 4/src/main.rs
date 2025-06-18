// File: main.rs

// Memasukkan file utils.rs sebagai modul
mod utils;

fn main() {
    utils::say_hello("Qinthar");

    let sum = utils::add(10, 5);
    println!("Hasil penjumlahan: {}", sum);
}
