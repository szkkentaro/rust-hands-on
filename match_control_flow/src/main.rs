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

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // ... etc
    }
    // an enum and a match
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("Staete quater from {:?}!", state);
                25
            }
        }
    }
    let p = Coin::Penny;
    let n = Coin::Nickel;
    let d = Coin::Dime;
    let q = Coin::Quarter(UsState::Alabama);
    println!("{}, {}, {}, {}", value_in_cents(p), value_in_cents(n), value_in_cents(d), value_in_cents(q));

    println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));


    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
            None => None, // this line is need avoid non-exhaustive patterns
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}, {:?}, {:?}", six, none, plus_one(six));
}
