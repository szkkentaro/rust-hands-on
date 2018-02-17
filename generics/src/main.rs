fn main() {

    let x: Option<i32> = Some(5);
    let y: Option<f64> = Some(5.0f64);
    println!("{:?}, {:?}", x, y);

    struct Point<T> {
        x: T,
        y: T
    }
    impl<T> Point<T> {
        fn swap(&mut self) {
            std::mem::swap(&mut self.x, &mut self.y);
        }
    }

    let mut point = Point { x: Some(1), y: Some(2)} ;
    point.swap();

    println!("Swaped values x:{:?}, y:{:?}", point.x, point.y);

    // multiple generics
    struct Foo<T, U> {
        x: T,
        y: U,
    }
    impl<T, U> Foo<T, U> {
        fn mixup<V, W>(self, other: Foo<V, W>) -> Foo<T, W> {
            Foo {
                x: self.x,
                y: other.y
            }
        }
    }
    let f1 = Foo { x: 5, y: 10.5 };
    let f2 = Foo { x: "Hello", y: 'c' };
    let f = f1.mixup(f2);
    println!("f.x = {}, f.y = {}", f.x, f.y);
}
