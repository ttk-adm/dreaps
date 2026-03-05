pub fn xfit(x: &[f64]) -> (f64, f64) {
    use crate::math::{mean, stdev};
    let xmean: f64 = mean(x);
    let sigma: f64 = stdev(x);
    (xmean, sigma)
}

pub fn xfit_weighted(x: &[f64], weights: &[f64]) -> (f64, f64) {
    use crate::math::{mean_weighted, stdev_weighted};
    let xmean: f64 = mean_weighted(x, weights);
    let sigma: f64 = stdev_weighted(x, weights);
    (xmean, sigma)
}
