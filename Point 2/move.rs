fn main() {
    let s1 = String::from("Rust");
    let s2 = s1; // ownership pindah ke s2, s1 tidak bisa digunakan lagi
    println!("{}", s2);
    // println!("{}", s1); // ERROR: value borrowed after move
}
