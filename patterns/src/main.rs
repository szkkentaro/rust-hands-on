fn main() {
    let x = "x";
    let c = "c";
    match c {
        // shadowing
        x => println!("x: {}, c: {}", x, c),
    }
    println!("x: {}", x);

    // multiple
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        _ => println!("anything"),
    }

    struct Point {
        x: i32,
        y: i32,
    }
    let origin = Point {x: 0, y: 0};

    match origin {
        Point {x, y} => println!("{}, {}", x, y),
    }
    match origin {
        Point {x: x1, y: y1} => println!("{}, {}", x1, y1),
    }
    match origin {
        Point {y, .. } => println!("y is {}", y),
    }
}
