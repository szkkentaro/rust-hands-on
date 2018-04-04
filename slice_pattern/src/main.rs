#![feature(slice_pattern)]

fn main() {
    let v = vec!["match_this", "1"];
    match &v[..] {
        ["match_this", second] => println!("This is second value: {}", second),
        _ => {},
    }
}
