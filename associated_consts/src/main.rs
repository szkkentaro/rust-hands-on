trait Foo {
    const ID: i32;
}

impl Foo for i32 {
    const ID: i32 = 1;
}

impl Foo for i64 {
    const ID: i32 = 5;
}

struct Bar;

impl Bar {
    const ID: i32 = 100;
}

fn main() {
    assert_eq!(1, i32::ID);
    assert_eq!(5, i64::ID);
    println!("{}", Bar::ID);
}