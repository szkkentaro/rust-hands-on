fn main() {
    // without return type
    let plus_one = |x: i32| x + 1;
    assert_eq!(2, plus_one(1));

    // with return type
    let plus_two = |x| -> i32 {
        let mut result: i32 = x;
        result += 1;
        result += 1;
        result
    };
    assert_eq!(3, plus_two(1));

    // bindding
    let num = 5;
    let plus_num = |x: i32| x + num;
    assert_eq!(10, plus_num(5));

    // move closure
    let owns_num = move |x: i32| x + num;
    assert_eq!(10, owns_num(5));

    let foo = 5;
    {
        let add_num = move |x: i32| x + foo;
        add_num(5);
    }
    // because of 5 has Copy method
    assert_eq!(5, foo);

    // closure as a arg, then this func is static dispatched
    fn call_with_one<F>(some_closure: F) -> i32
        where F : Fn(i32) -> i32 {
        some_closure(1)
    }
    let answer = call_with_one(|x| x + 2);
    assert_eq!(3, answer);

    // closure as a arg, then this func is dynamic dispatched
    fn call_with_two(some_closuer: &Fn(i32) -> i32) -> i32 {
        some_closuer(2)
    }
    let answer = call_with_two(&|x:i32| x + 2);
    assert_eq!(4, answer);


    // the pointer of closuer func is used as a arg
    fn add_two(i: i32) -> i32 {
        i + 2
    }
    let answer = call_with_two(&add_two);
    assert_eq!(4, answer);
    
    // return closure
    // point
    // - use Box because of lifetime problem
    // - use move for closure, because it has borrow num
    fn factory() -> Box<Fn(i32) -> i32> {
        let num = 5;
        Box::new(move |x| x + num)
    }
    let f = factory();
    let answer = f(1);
    assert_eq!(6, answer);
    
}
