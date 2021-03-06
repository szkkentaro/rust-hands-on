struct Point {
    x: i32,
    y: i32
    // z: mut i32 : can not make field mutable
}

struct Point3d {
    x: i32,
    y: i32,
    z: i32
}

// tuple struct
struct Color(i32, i32, i32);
struct Inches(i32);

// unit-like struct
struct Electron;

fn main() {
    let mut p = Point {x: 0, y: 0};
    p.x = 5;
    
    let p = p; // this trick make field mutable
    // p.y = 6; // can not mutably borrow immutable field

    println!("p is at ({}, {})", p.x, p.y);

    // Update
    let mut point = Point3d{x: 0, y: 0, z: 0};
    point = Point3d {y: 1, .. point};
    println!("{}, {}, {}", point.x, point.y, point.z);

    let src = Point3d{x: 0, y: 0, z: 0};
    let dst = Point3d{z: 1, x: 2, .. src};
    println!("{}, {}, {}", src.x, src.y, src.z);
    println!("{}, {}, {}", dst.x, dst.y, dst.z);

    // tuple struct 
    let black = Color(0, 0, 0);
    println!("{}, {}, {}", black.0, black.1, black.2);
    // tuple struct : newtype pattern
    let length = Inches(10);
    let Inches(integer_length) = length;
    println!("interger_length is {}", integer_length);
    
    // ToDo: What is this useful?
    let _x = Electron;
}
