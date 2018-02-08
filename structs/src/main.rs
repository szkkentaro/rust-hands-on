struct Point {
    x: i32,
    y: i32
    // z: mut i32 : can not make field mutable
}

fn main() {
    let mut p = Point {x: 0, y: 0};
    p.x = 5;
    
    let p = p; // this trick make field mutable
    p.y = 6; // can not mutably borrow immutable field

    println!("p is at ({}, {})", p.x, p.y);
}
