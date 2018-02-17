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
}
