fn main() {
    // as is safe
    let x: i32 = 5;
    let y = x as i64;
    println!("{}", y);

    let one = true as u8;
    let at_sign = 64 as char;
    let two_hundred = -56i8 as u8;
    println!("{}, {}, {}", one, at_sign, two_hundred);

    // transnute is danger

}
