fn main() {
    fn foo(x: i32) {
        println!("{}", x);
    }

    let opt: Option<i32> = Some(5);
    if let Some(x) = opt {
        foo(x);
    }
}
