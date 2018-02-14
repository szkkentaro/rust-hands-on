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

}

