use std::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

fn main() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    for i in 0..3 {
        let data = data.clone();
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data[i] += 1;
        });
    }
    thread::sleep(Duration::from_millis(50));

    let data = Arc::new(Mutex::new(0));
    let (tx, rx) = mpsc::channel();
    for _ in 0..10 {
        let (data, tx) = (data.clone(), tx.clone());
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            *data += 1;
            tx.send(()).unwrap();
        });
    }
    for _ in 0..10 {
        rx.recv().unwrap();
    }

    let (tx, rx) = mpsc::channel();
    for i in 0..10 {
        let tx = tx.clone();
        thread::spawn(move || {
            let answer = i * i;

            tx.send(answer).unwrap();
        });
    }
    for _ in 0..10 {
        println!("{:?}", rx.recv().unwrap());
    }

    let handle = thread::spawn(|| {
        panic!("oops!");
    });

    let result = handle.join();
    assert!(result.is_err());
}
