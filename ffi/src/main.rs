extern crate libc;
// use libc::size_t;

extern "C" fn callback(a: i32) {
    println!("callback from {0}", a);
}

#[link(name = "extlib")]
extern "C" {
    fn register_callback(cb: extern "C" fn(i32)) -> i32;
    fn trigger_callback();
}

// #[link(name = "snappy")]
// extern "C" {
//     fn snappy_max_compressed_length(source_length: size_t) -> size_t;
// }

fn main() {
    // let x = unsafe { snappy_max_compressed_length(100) };
    // println!("max compressed length of a 100 byte buffer: {}", x);
    unsafe {
        register_callback(callback);
        trigger_callback();
    }
}
