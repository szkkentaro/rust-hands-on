#![feature(slice_patterns)]

fn is_symmetric(list: &[u32]) -> bool {
    match list {
        [] | [_] => true,
        [x, inside.., y] if x == y => is_symmetric(inside),
        _ => false,
    }
}

fn main() {
    let v = vec!["match_this", "1"];
    match &v[..] {
        ["match_this", second] => println!("This is second value: {}", second),
        _ => {},
    }

    let sym = &[0,1,2,4,2,1,0];
    assert!(is_symmetric(sym));
}