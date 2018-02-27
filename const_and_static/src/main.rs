fn main() {
    const FOO: i32 = 5;
    assert_eq!(5, FOO);

    static BAR: &'static str = "Bar";
    assert_eq!("Bar", BAR);
}
