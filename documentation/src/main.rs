fn main() {
    #[derive(Debug)]
    struct Foo {
        x: i32,
    }

    /// Summary
    ///
    /// Description1
    /// Description2
    ///
    /// # Examples
    ///
    /// standard usage
    ///
    /// ```
    /// let f = new(10);
    /// ```
    ///
    fn new(x: i32) -> Foo {
        Foo { x: x }
    }

    println!("{:?}", new(10));
}
