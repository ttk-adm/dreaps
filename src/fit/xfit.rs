pub fn xfit(x: &[f64]) -> (f64, f64) {
    use crate::math::stats::{mean, stdev};
    let xmean: f64 = mean(x);
    let sigma: f64 = stdev(x);
    (xmean, sigma)
}

pub fn xfit_weighted(x: &[f64], weights: &[f64]) -> (f64, f64) {
    use crate::math::stats::{mean_weighted, stdev_weighted};
    let xmean: f64 = mean_weighted(x, weights);
    let sigma: f64 = stdev_weighted(x, weights);
    (xmean, sigma)
}

#[cfg(test)]
mod tests {
    use super::*;
     #[test]
     fn test_xfit() {
        let small_vec: Vec<f64> = vec![1., 2., 3.];
        assert_eq!(xfit(&small_vec), (2., 1.));
     }
}
