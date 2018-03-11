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
}
