fn main() {
    // type annotation
    let guess: u32 = "42".parse().expect("Not a number");
    println!("{}", guess);

    // floating point
    let x = 2.0;
    let y: f32 = 3.0;
    println!("{}, {}", x, y); 

}
