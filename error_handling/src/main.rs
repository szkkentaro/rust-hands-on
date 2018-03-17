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

fn file_path_ext_explicite(file_path: &str) -> Option<&str> {
    match file_name(file_path) {
        None => None,
        Some(name) => match extension(name) {
            None => None,
            Some(ext) => Some(ext),
        },
    }
}

#[allow(unused_variables)]
fn file_name(file_path: &str) -> Option<&str> {
    let mut i: usize = 0;
    for (offset, c) in file_path.char_indices() {
        if '/' == c {
            i = offset;
        }
    }
    Some(&file_path[i + 1..])
}

fn file_path_ext(file_path: &str) -> Option<&str> {
    file_name(file_path).and_then(extension)
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

    let file_path = "/path/to/file.md";
    println!("{:?}", file_path_ext_explicite(file_path));
    println!("{:?}", file_path_ext(file_path));
}
