#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        assert!(false);
    }
}
