#[cfg(test)]
mod test {
    use crate::math::discrete;

    #[test]
    fn test_fact_u64() {
        assert_eq!(discrete::fact_u64(1), Some(1));
        assert_eq!(discrete::fact_u64(3), Some(6));
        assert_eq!(discrete::fact_u64(4), Some(24));
        assert_eq!(discrete::fact_u64(21), None);
    }

    #[test]
    fn test_fact_u128() {
        assert_eq!(discrete::fact_u128(1), Some(1));
        assert_eq!(discrete::fact_u128(3), Some(6));
        assert_eq!(discrete::fact_u128(4), Some(24));
        assert_eq!(discrete::fact_u128(21), Some(51090942171709440000));
        assert_eq!(discrete::fact_u128(35), None);
    }

    #[test]
    fn test_log_sum() {
        assert_eq!(discrete::log_sum(1), 0.);
        assert_eq!(discrete::log_sum(2), 0.6931471805599453);
        assert_eq!(discrete::log_sum(3), 1.791759469228055);
        assert_eq!(discrete::log_sum(35), 92.13617560368711);
    }

    #[test]
    fn test_fact_f64() {
        assert_eq!(discrete::fact_f64(1), 1.);
        assert_eq!(discrete::fact_f64(2), 2.);
        assert_eq!(discrete::fact_f64(3), 6.);
        assert_eq!(discrete::fact_f64(4), 24.);
        assert_eq!(discrete::fact_f64(21), 51090942171709440000.);
        assert_eq!(
            discrete::fact_f64(35),
            10333147966386297000000000000000000000000.
        );
    }

    #[test]
    fn test_binomial_permutations() {
        assert_eq!(discrete::binomial_permutations(3, 3), 1.);
        assert_eq!(discrete::binomial_permutations(3, 1), 2.9999999999999996);
        assert_eq!(discrete::binomial_permutations(4, 1), 3.999999999999999);
        assert_eq!(discrete::binomial_permutations(4, 2), 5.999999999999997);
        assert_eq!(discrete::binomial_permutations(4, 3), 3.999999999999999);
    }
}
