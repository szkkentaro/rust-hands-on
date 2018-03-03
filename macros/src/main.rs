macro_rules! foo {
    (x => $e:expr) => (println!("mode X: {}", $e));
}
fn main() {
    foo!(x => 3);
    // foo!(y => 3); // compile error: No rules expected the token `y`
}
