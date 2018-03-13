use std::thread;

fn main() {
    let handle = thread::spawn(|| "Hello from thread!");
    println!("{}", handle.join().unwrap());
}
