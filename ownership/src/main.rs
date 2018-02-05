fn main() {
    // ownership and functions
    let s = String::from("hello"); // s comes into scope
    takes_ownership(s);            // s's value moves into the function...
                                   // ... and so is nolonger valid here.
    //println!("{}", s);           // Error: use of moved value

    let x = 5;                     // x comes into scope
    makes_copy(x);                 // x moves into the function,
                                   // but, i32 is Copy, so it's okay to still
                                   // use x afterward.

    // primitive value has Copy type
    let a = 5;
    let _y = double(a);
    println!("{}", a);

    let a = true;
    let _y = change_truth(a);
    println!("{}", a);

    // borrowing
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];
    let answer = foo(&v1, &v2);
    println!("{}", answer);

    // &mut
    let mut x  = 5;
    {
        let y = &mut x;
        *y += 1;
    }
    println!("{}", x);

}
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    v1[0] + v2[0] // return answer
}

fn double(x: i32) -> i32 {
    x * 2
}

fn change_truth(x: bool) -> bool {
    !x
}