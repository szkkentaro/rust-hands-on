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

}
