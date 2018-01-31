fn main() {
    let number = 6;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than 0");
    }

    if number % 4 == 0 {
        println!("number is divided by 4");
    } else if number % 3 == 0 {
        println!("number is divided by 3");
    } else if number % 2 == 0 {
        println!("number is divided by 2");
    } else {
        println!("number is not divided by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        6
        // variables must have a single type
        // "six"
    };

    println!("The number of condition is {}", number);

    // loop
    let mut cnt = 0;
    loop {
        if cnt >= 3 {
            break;
        }
        println!("again!");
        cnt += 1;
    }

    // while
    let mut x = 3;
    while x != 0 {
        println!("x is {}", x);
        x = x - 1
    }
    println!("LIFTOFF!");

    // for
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("The value is {}", element);
    }

    // array reverse
    for n in (1..4).rev() {
        println!("n is {}", n);
    }
    println!("LIFTOFF!");
}
