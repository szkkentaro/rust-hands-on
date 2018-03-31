#![feature(test)]

extern crate test;

pub fn add_two(n: i32) -> i32 {
    n + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| add_two(2));
    }

    #[bench]
    fn bench_xor_1000_ints(b: &mut Bencher) {
        b.iter(|| (0..1000).fold(0, |old, new| old ^ new));
    }
}
