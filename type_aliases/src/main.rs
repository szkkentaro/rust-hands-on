fn main() {
    type Name = String;

    let s: Name = "Steve".to_string();
    println!("{}", s);

    type Num = i32;
    let x: i32 = 5;
    let y: Num = 5; // same as i32
    if x == y {     // then, there is no compile error of type check
        println!("{}", x + y);
    }
}
