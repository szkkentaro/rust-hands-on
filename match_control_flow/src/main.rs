fn main() {
    let x = 3;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("something else")
    }
    let num = match x {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "something else"
    };
    println!("{}", num);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => () // do nothing
    }
}
