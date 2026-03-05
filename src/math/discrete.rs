pub fn fact_u64(n: u64) -> Option<u64> {
    if n > 20 {
        return None;
    }
    (1..=n).try_fold(1, |acc: u64, x: u64| acc.checked_mul(x))
}

pub fn fact_u128(n: u64) -> Option<u128> {
    if n > 34 {
        return None;
    }
    (1..=n as u128).try_fold(1, |acc: u128, x: u128| acc.checked_mul(x))
}

pub fn log_sum(n: u64) -> f64 {
    (2..=n).map(|x| (x as f64).ln()).sum()
}

pub fn fact_f64(n: u64) -> f64 {
    if let Some(fact) = fact_u64(n) {
        return fact as f64;
    } else if let Some(fact) = fact_u128(n) {
        return fact as f64;
    } else {
        log_sum(n).exp()
    }
}

pub fn binomial_permutations(n: u64, x: u64) -> f64 {
    let nsum = log_sum(n);
    let xsum = log_sum(x);
    let zsum = log_sum(n - x);
    let sum = nsum - xsum - zsum;
    sum.exp()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fact_f64() {
        assert_eq!(fact_f64(2), 2.0);
        assert_eq!(fact_f64(3), 6.0);
        assert_eq!(fact_f64(4), 24.0);
        assert_eq!(fact_f64(5), 120.0);
    }
}
