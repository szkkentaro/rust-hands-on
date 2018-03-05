fn main() {
    let x = 5;
    let raw = &x as *const i32;
    let points_at = unsafe { *raw };

    let mut y = 10;
    let raw_mut = &mut y as *mut i32;

    println!("{}, {:?}, {}", x, raw, points_at);
    println!("{}, {:?}", y, raw_mut);
}
