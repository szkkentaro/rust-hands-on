// use std::collections::HashMap;
use std::borrow::Borrow;
use std::fmt::Display;

fn main() {
    fn foo<T: Borrow<i32> + Display>(a: T) {
        println!("a is borrowd: {}", a);
    }

    let mut i = 5;
    foo(&i);
    foo(&mut i);

    // let mut map = HashMap::new();
    // map.insert("Foo".to_string(), 42);
    // assert_eq!(map.get("Foo"), Some(&42));

    let s = "Hello".to_string();
    fn bar<T: AsRef<str>>(s: T) {
        let slice = s.as_ref();
        println!("s is comverted reference: {}", slice);
    }
    bar(s);
}
