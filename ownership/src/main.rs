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

    // return values and scope
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("{}, {}", s1, s3);

    // return multiple values
    let s1 = String::from("hello");
    let (s2, len) = calcurate_length(s1);
    println!("The length of '{}' is {}.", s2, len);

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

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calcurate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
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