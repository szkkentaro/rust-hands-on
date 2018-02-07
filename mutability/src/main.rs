struct Point {
    x: i32,
    y: i32
}

fn main() {
    let mut a = Point { x: 5, y: 6 };
    a.x = 10;
    let b = Point { x: 5, y: 6 };
    //b.x = 10; // cannot mutably borrow immutable field
    println!("{}, {}", b.x, b.y);
}