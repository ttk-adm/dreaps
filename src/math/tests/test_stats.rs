#[cfg(test)]
mod test {
    use crate::math::stats::square_diff;

    #[test]
    fn test_square_diff() {
        assert_eq!(square_diff(10., 10.), 0.);
        assert_eq!(square_diff(2., 1.), 1.);
        assert_eq!(square_diff(1., 2.), 1.);
        assert_eq!(square_diff(4., 1.), 9.);
        assert_eq!(square_diff(1., 4.), 9.);
    }
}
