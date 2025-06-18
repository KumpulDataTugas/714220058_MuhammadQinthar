struct Point {
    x: i32,
    y: i32,
}

fn move_point(p: Point, dx: i32, dy: i32) -> Point {
    Point {
        x: p.x + dx,
        y: p.y + dy,
    }
}

fn main() {
    let p = Point { x: 1, y: 2 };
    let p2 = move_point(p, 5, 3);
    println!("New Point: ({}, {})", p2.x, p2.y);
}
