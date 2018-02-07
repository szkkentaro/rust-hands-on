use std::cell::Cell;

struct Point {
    x: i32,
    y: i32
}
struct PointWithCell {
    x: i32,
    y: Cell<i32>
}

fn main() {
    let mut a = Point { x: 5, y: 6 };
    a.x = 10;
    let b = Point { x: 5, y: 6 };
    //b.x = 10; // cannot mutably borrow immutable field
    println!("x: {}, y: {}", b.x, b.y);
    let c = PointWithCell {x: 5, y: Cell::new(6)};
    c.y.set(7);

    println!("x: {}, y: {:?}", c.x, c.y);
}