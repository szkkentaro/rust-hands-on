fn main() {
    // This is statement
    let x = 5;

    // This is expression
    let y = {
        let x = 3;
        x + 1
    };
    println!("{}, {}", x, y);
}