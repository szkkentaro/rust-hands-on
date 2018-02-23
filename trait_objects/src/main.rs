trait Foo {
    fn method(&self) -> String;
}
impl Foo for u8 {
    fn method(&self) -> String {
        format!("u8: {}", *self)
    }
}
impl Foo for String {
    fn method(&self) -> String {
        format!("String: {}", *self)
    }
}

// static dispacth
fn do_something<T: Foo>(x: T) -> String{
    x.method()
}

// dynamic dispatch
fn do_something_dynamic_dispatch(x: &Foo) -> String {
    x.method()
}

fn main() {
    let x = 5u8;
    println!("{}", do_something(x));

    let y = "Hello".to_string();
    println!("{}", do_something(y));

    let z = 5u8;
    do_something_dynamic_dispatch(&z as &Foo);
    println!("{}", do_something_dynamic_dispatch(&z as &Foo));
    println!("{}", do_something_dynamic_dispatch(&z)); // type coercion
}
