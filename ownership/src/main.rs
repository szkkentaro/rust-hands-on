fn main() {
    println!("Hello, world!");
}


fn foo() {
    let v = vec![1, 2, 3];
    let v2 = v;
    println!("v[0] is : {}:", v[0]); // error: use of moved value
}