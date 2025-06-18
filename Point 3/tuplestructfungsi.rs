struct Color(u8, u8, u8);

fn show_color(c: &Color) {
    println!("RGB({}, {}, {})", c.0, c.1, c.2);
}

fn main() {
    let blue = Color(0, 0, 255);
    show_color(&blue);
}
