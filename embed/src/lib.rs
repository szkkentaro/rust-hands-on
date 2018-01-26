use std::thread;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[no_mangle]
pub extern fn process() {
    let headles: Vec<_> = (0..10).map(|_| {
        thread::spawn(|| {
            let mut x = 0;
            for _ in 0..5_000_000 {
               x += 1 
            }
            x
        })
    }).collect();

    for h in headles {
        println!("Thread finished with count={}", 
        h.join().map_err(|_| "Cloud not join a thread!").unwrap());
    }
}