#[cfg(any(not(unix), all(target_os = "macos", target_arch = "x86_64")))]
fn main() {
    println!("Hello, world!");

    if cfg!(target_os = "macos") || cfg!(target_os = "ios") {
        println!("Think Different!");
    }
}
