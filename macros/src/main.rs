// Matching
macro_rules! foo {
    (
        x => $e:expr
    ) => (
        println!("mode X: {}", $e)
    );
}

// Repetition
macro_rules! o_O {
    (
        $(
            $x:expr; [ $($y:expr),* ]
        );*
    ) => (
        &[ $($($x + $y),*),* ]
    )
}
fn main() {
    foo!(x => 3);
    // foo!(y => 3); // compile error: No rules expected the token `y`

    let a: &[i32] = o_O!(10; [1,2,3];
                         20; [4,5,6]);
    assert_eq!(a, [11, 12, 13, 24, 25, 26]);
}
