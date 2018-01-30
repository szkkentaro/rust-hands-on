fn main() {
    // type annotation
    let guess: u32 = "42".parse().expect("Not a number");
    println!("{}", guess);

    // floating point
    let x = 2.0;
    let y: f32 = 3.0;
    println!("{}, {}", x, y); 

    // addtion
    let sum = 5 + 10;
    // substruction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;
    println!("{}, {}, {}, {}, {}", sum, difference, product, quotient, remainder);

    // boolean
    let t = true;
    let f: bool = false;
    println!("{},{}", t, f);

    // character
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{},{},{}", c, z, heart_eyed_cat);
}
