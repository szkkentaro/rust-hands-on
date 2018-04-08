#![feature(alloc_system)]

extern crate alloc_system;

fn main() {
    let a = Box::new(4);
    println!("{}", a);
}