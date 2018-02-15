fn main() {
    let greeting = "Hello world!"; // &'static str type
    println!("{}", greeting);

    let s = "foo
    bar";
    assert_eq!("foo\n    bar", s);

    let s = "foo\
    bar";
    assert_eq!("foobar", s);

    let mut s = "Hello".to_string(); // mut s is String
    println!("{}", s);
    s.push_str(", world");
    println!("{}", s);

    fn take_slice(slice: &str) {
        println!("{}", slice);
    }
    let x = "Hey";
    take_slice(x);

    // Indexing
    let skytree = "東京スカイツリー";
    for b in skytree.as_bytes() {
        print!("{}, ", b);
    }
    println!("");
    for c in skytree.chars() {
        print!("{}, ", c);
    }

    let tokyo = &skytree[0..6]; // it refeters bytes length
    println!("{}", tokyo);

    // concat String + &str
    let good = "good".to_string();
    let morning = "morning";
    println!("{}", good + morning);

    // concat String + String
    let good = "good".to_string();
    let morning = "morning".to_string(); // it will transform &str forcely
    println!("{}", good + &morning);
}

