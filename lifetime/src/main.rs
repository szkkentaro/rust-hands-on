struct Foo<'a> {
    x: &'a i32,
}

impl<'a> Foo<'a> {
    fn x(&self) -> &'a i32 { self.x }
}

fn main() {
    let y = &5; // same as let _y = 5; let y = &_y;
    let f = Foo{x: y};

    println!("{}", f.x());

    let x: &'static str = "Hello world."; // Strtig literal
    static FOO: i32 = 5;
    let y: &'static i32 = &FOO;

    println!("{}, {}", x, y);
}
