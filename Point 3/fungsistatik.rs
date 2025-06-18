struct Circle {
    radius: f64,
}

impl Circle {
    fn new(r: f64) -> Circle {
        Circle { radius: r }
    }

    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}

fn main() {
    let c = Circle::new(3.0);
    println!("Circle area: {:.2}", c.area());
}
