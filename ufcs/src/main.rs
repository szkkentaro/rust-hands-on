trait Foo {
    fn f(&self);
    fn num() -> i32;
}

trait Bar {
    fn f(&self);
}

struct Baz;
impl Baz {
    fn num() -> i32 {
        20
    }
}
impl Foo for Baz {
    fn f(&self) {
        println!("Foo for Baz");
    }
    
    fn num() -> i32 {
        10
    }
}
impl Bar for Baz {
    fn f(&self) {
        println!("Bar for Baz");
    }
}

fn main() {
    let b = Baz;
    Foo::f(&b);
    Bar::f(&b);

    assert_eq!(10, <Baz as Foo>::num());
    assert_eq!(20, Baz::num());
    
}
