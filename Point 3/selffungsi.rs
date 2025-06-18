struct Car {
    model: String,
}

impl Car {
    fn describe(self) {
        println!("Car model: {}", self.model);
    }
}

fn main() {
    let car = Car {
        model: String::from("Toyota"),
    };
    car.describe(); // self dikonsumsi di sini
}
