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

    // an enum and a match
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
    let p = Coin::Penny;
    let n = Coin::Nickel;
    let d = Coin::Dime;
    let q = Coin::Quarter;
    println!("{}, {}, {}, {}", value_in_cents(p), value_in_cents(n), value_in_cents(d), value_in_cents(q));
}
