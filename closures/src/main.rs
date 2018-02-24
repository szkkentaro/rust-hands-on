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

}
