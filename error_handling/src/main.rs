#[allow(unused_imports)]
use std::env;

use std::num::ParseIntError;
use std::path::Path;
use std::io::Read;
use std::fs::File;

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

fn double_number(num_str: &str) -> Result<i32, ParseIntError> {
    num_str.parse::<i32>().map(|x| x * 2)
}

fn double_arg(mut argv: env::Args) -> Result<i32, String> {
    argv.nth(1)
        .ok_or("Please give at least one arg".to_owned())
        .and_then(|arg| arg.parse::<i32>().map_err(|err| err.to_string()))
        .map(|n| n * 2)
}

fn file_double<P: AsRef<Path>>(file_name: P) -> Result<i32, String> {
    let mut file = try!(File::open(file_name).map_err(|e| e.to_string()));
    let mut contents = String::new();
    try!(
        file.read_to_string(&mut contents)
            .map_err(|e| e.to_string())
    );
    let n = try!(contents.trim().parse::<i32>().map_err(|e| e.to_string()));
    Ok(n * 2)
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

    match double_number("10") {
        Ok(n) => assert_eq!(n, 20),
        Err(err) => println!("{:?}", err),
    }

    /*
    $ error_handling/target/debug/error_handling 10
        File extension: rs
        Some("rs")
        Some("md")
        Some("md")
        20
    */
    match double_arg(env::args()) {
        Ok(n) => println!("{}", n),
        Err(err) => println!("with error: {}", err),
    }

    /*
    $ cd path/to/debug
      ./error_handling 10
      File extension: rs
        Some("rs")
        Some("md")
        Some("md")
        20
        4
    */
    match file_double("foobar") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("{}", err),
    }
}
