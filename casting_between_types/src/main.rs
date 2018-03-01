fn main() {
    // as is safe
    let x: i32 = 5;
    let y = x as i64;
    println!("{}", y);

    let one = true as u8;
    let at_sign = 64 as char;
    let two_hundred = -56i8 as u8;
    println!("{}, {}, {}", one, at_sign, two_hundred);

    // pointer casts
    let a = 300 as *const char;
    let b = a as u32;
    println!("{:?}, {}", a, b);

    // transnute is danger

}
