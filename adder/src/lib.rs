pub fn add_two(x: i32) -> i32 {
    x + 2
}

#[cfg(test)]
mod tests {
    pub fn add_two(x: i32) -> i32 {
        x + 2
    }

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn it_works_other() {
        assert_eq!("Hello", "World");
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        assert!(true);
    }
}
