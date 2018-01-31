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
}
