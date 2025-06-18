// File: utils.rs

// Fungsi publik agar bisa dipanggil dari luar file ini
pub fn say_hello(name: &str) {
    println!("Hello, {}!", name);
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
