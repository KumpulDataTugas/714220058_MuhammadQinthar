struct Counter {
    value: i32,
}

impl Counter {
    fn increment(&mut self) {
        self.value += 1;
    }
}

fn main() {
    let mut count = Counter { value: 0 };
    count.increment();
    println!("Counter: {}", count.value);
}
