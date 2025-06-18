struct Person {
    name: String,
    age: u8,
}

fn print_person(p: &Person) {
    println!("{} is {} years old.", p.name, p.age);
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    print_person(&person);
}
