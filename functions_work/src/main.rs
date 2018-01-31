fn main() {
    // This is statement
    let x = 5;

    // This is expression
    let y = {
        let x = 3;
        x + 1
    };
    println!("{}, {}", x, y);

    let x = plus_one(5);
    println!("{}", x);

    //diverges()

    // function pointer

    //let f: fn(i32) -> i32 = plus_one;
    let f = plus_one; // with type inference
    println!("{}", f(9));
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// fn diverges() -> ! {
//     panic!("This function never returns!");
// }

