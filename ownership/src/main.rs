fn main() {
    // primitive value has Copy type
    let a = 5;
    let _y = double(a);
    println!("{}", a);

    let a = true;
    let _y = change_truth(a);
    println!("{}", a);

    // borrowing
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];
    let answer = foo(&v1, &v2);
    println!("{}", answer);

    // &mut
    let mut x  = 5;
    {
        let y = &mut x;
        *y += 1;
    }
    println!("{}", x);

}

fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    v1[0] + v2[0] // return answer
}

fn double(x: i32) -> i32 {
    x * 2
}

fn change_truth(x: bool) -> bool {
    !x
}