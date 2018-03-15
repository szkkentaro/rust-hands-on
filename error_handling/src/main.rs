#[allow(unused_imports)]
use std::env;

#[allow(unused_variables, dead_code)]
fn guess(i: i32) -> bool {
    if i < 0 || i > 10 {
        panic!("Invalid number: {}", i);
    }
    i == 5
}

fn find(haystack: &str, needle: char) -> Option<usize> {
    for (offset, c) in haystack.char_indices() {
        if needle == c {
            return Some(offset);
        }
    }
    None
}

fn extension_explicit(file_name: &str) -> Option<&str> {
    match find(file_name, '.') {
        None => None,
        Some(i) => Some(&file_name[i + 1..]),
    }
}

fn extension(file_name: &str) -> Option<&str> {
    // use map conbinator
    find(file_name, '.').map(|i| &file_name[i + 1..])
}

fn main() {
    // The Basics
    // guess(11);

    // The Basics 2
    // let mut arg = env::args();
    // let arg: String = arg.nth(1).unwrap(); // error 1
    // let n: i32 = arg.parse().unwrap(); // error 2
    // println!("{}", n * 2);

    let file_name = "foobar.rs";
    match find(file_name, '.') {
        None => println!("No file extension found."),
        Some(i) => println!("File extension: {}", &file_name[i + 1..]),
    }

    println!("{:?}", extension_explicit(file_name));

    let csv_file_name = "foobar.csv";
    let file_name = "foobar";
    assert_eq!(extension(csv_file_name).unwrap_or("rs"), "csv");
    assert_eq!(extension(file_name).unwrap_or("rs"), "rs");
}
