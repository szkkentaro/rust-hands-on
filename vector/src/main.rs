fn main() {
    let v = vec![0; 3];
    for i in &v {
        println!("A reference to {}", i);
    }
    println!("Vector index 1 is {}", v[1]);

    let mut v = vec![1,2,3];
    for i in &mut v {
        println!("A mutable reference to {}", i);
    }
    for i in v {
        println!("Take ownership of the vector and its element {}", i);
    }
}
