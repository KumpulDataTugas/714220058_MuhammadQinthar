fn main() {
    {
        let s = String::from("In Scope");
        println!("{}", s);
    }
    // println!("{}", s); // ERROR: s sudah di luar scope
}
