#[allow(unused_variables)]
fn main() {
    let mut range = 0..10;
    loop {
        match range.next() {
            Some(x) => {
                println!("{}", x);
            }
            None => break,
        }
    }

    let nums = vec![1, 2, 3];

    // this is not recommend
    for i in 0..nums.len() {
        println!("{}", nums[i]);
    }

    // you should do below
    for num in &nums {
        println!("{}", num);
    }

    // consumer with type placeholder
    let one_to_one_hundred = (1..101).collect::<Vec<_>>();

    // find consumer
    let greater_than_forty_two = (0..100).find(|x| *x > 42);

    match greater_than_forty_two {
        Some(_) => println!("Found a match!"),
        None => println!("No match found"),
    }

    // fold consumer
    let sum = (1..11).fold(0, |sum, x| sum + x);
    println!("{}", sum);

    // iterator
    let nums = vec![1, 2, 3];
    for num in nums.iter() {
        println!("{}", num);
    }

    // take() returns iterator
    for i in (4..).take(6) {
        println!("{}", i);
    }
}
