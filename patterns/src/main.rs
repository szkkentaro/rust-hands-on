fn main() {
    let x = "x";
    let c = "c";
    match c {
        // shadowing
        x => println!("x: {}, c: {}", x, c),
    }
    println!("x: {}", x);

    // multiple
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        _ => println!("anything"),
    }

    struct Point {
        x: i32,
        y: i32,
    }
    let origin = Point {x: 0, y: 0};

    match origin {
        Point {x, y} => println!("{}, {}", x, y),
    }
    match origin {
        Point {x: x1, y: y1} => println!("{}, {}", x1, y1),
    }
    match origin {
        Point {y, .. } => println!("y is {}", y),
    }

    // ignore variable binding
    let some_value: Result<&'static str, &'static str> = Ok("Hello world");
    match some_value {
        Ok(value) => println!("got a value: {}", value),
        Err(_) => println!("an error occurred"),
    }
    
    fn cordinate() -> (i32, i32, i32) {
        (0, 1, 2)
    }
    let (x, _, z) = cordinate();
    println!("x: {}, z:{}", x, z);

    enum OptionalTuple {
        Value(i32, i32, i32),
        Missing,
    }
    let x = OptionalTuple::Value(5, -2, 3);
    match x {
        OptionalTuple::Value(..) => println!("got a tuple!"),
        OptionalTuple::Missing => println!("No such luck."),
    }

    // ref and ref mut
    let x = 5;
    match x {
        ref mr => println!("Got a reference to {}", mr),
    }
    let mut x = 5;
    match x {
        ref mut mr => println!("Got a mutable reference to {}", mr),
    }

    let y = 1;
    match y {
        1...5 => println!("one through five"),
        _ => println!("something else"),
    }

    let letter = 'c';
    match letter {
        'a' ... 'j' => println!("early letter"),
        'k' ... 'z' => println!("late letter"),
        _ => println!("something else")
    }

    let letter = 'ｶ';
    match letter {
        'ｱ' ... 'ﾝ' => println!("{}", letter),
        _ => println!("something else")
    }

    // binding in match
    #[derive(Debug)]
    struct Person {
        name: Option<String>
    }

    let name = "Steve".to_string();
    let x: Option<Person> = Some(Person {name: Some(name)});
    match x {
        Some(Person { name: ref a @ Some(_), ..}) => println!("{:?}", a),
        _ => {}
    }

    let x = 8;
    match x {
        e @ 1 ... 5 | e @ 8 ... 10 => println!("{}", e),
        _ => println!("anything"),
    }
}