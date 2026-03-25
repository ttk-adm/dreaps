#[cfg(test)]
mod test {
    use crate::math::array::compare_len;

    #[test]
    fn test_square_diff() {
        let array_a = [1., 2., 3.];
        let array_b = [4., 5., 6.];
        let array_c = [1., 2., 3., 4.];
        assert_eq!(compare_len(&array_a, &array_b), Ok(()));
        assert_eq!(
            compare_len(&array_a, &array_c),
            Err("arrays must have equal lengths".into())
        );
    }
}
