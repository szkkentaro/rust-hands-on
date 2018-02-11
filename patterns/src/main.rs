fn main() {
    let x = "x";
    let c = "c";
    match c {
        // shadowing
        x => println!("x: {}, c: {}", x, c),
    }
    println!("x: {}", x);

    // multiple
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        _ => println!("anything"),
    }
}
