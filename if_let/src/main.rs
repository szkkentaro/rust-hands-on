fn main() {
    fn foo(x: i32) {
        println!("{}", x);
    }

    let opt: Option<i32> = Some(5);
    if let Some(x) = opt {
        foo(x);
    }

    let mut v = vec![1,3,5,7,11];
    // loop {
    //     match v.pop() {
    //         Some(x) => println!("{}", x),
    //         None => break,
    //     }
    // }
    while let Some(x) = v.pop() {
        println!("{}", x);
    }
}
