use std::ops::Deref;
struct DerefExample<T> {
    value: T,
}

impl<T> Deref for DerefExample<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}

struct Foo;
impl Foo {
    fn foo(&self) {
        println!("foo");
    }
}

fn main() {
    let x = DerefExample { value: 'a' };
    assert_eq!('a', *x);

    // Deref and method calls
    let f = &&Foo;
    f.foo();
    (&f).foo();
    (&&f).foo();
    (&&&&&&&&&&&&&&&&&&&&&f).foo();
}
