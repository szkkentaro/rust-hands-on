trait Foo {
    const ID: i32;
}

impl Foo for i32 {
    const ID: i32 = 1;
}

struct Bar;

impl Bar {
    const ID: i32 = 100;
}

fn main() {
    assert_eq!(1, i32::ID);
    println!("{}", Bar::ID);
}