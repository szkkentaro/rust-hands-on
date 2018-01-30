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

    // compound types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{}, {}, {}", tup.0, tup.1, tup.2);

    let (x, y, z) = tup;
    println!("{}, {}, {}", x, y, z);

    // array
    let a = [1, 2, 3];
    println!("{}, {}, {}", a[0], a[1], a[2]);

    // index out of bounds build:pass run:panic
    // let index = 10;
    // let _element = a[index];
}
