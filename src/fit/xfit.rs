use crate::math::stats::StatsArray1D;

pub fn xfit(x: &[f64]) -> (f64, f64) {
    use crate::math::stats::{mean, stdev};
    let xmean: f64 = mean(x);
    let sigma: f64 = stdev(x);
    (xmean, sigma)
}

pub fn xfit_weighted(array: StatsArray1D) -> (f64, f64) {
    use crate::math::stats::{mean_weighted, stdev_weighted};
    let xmean: f64 = mean_weighted(array.clone());
    let sigma: f64 = stdev_weighted(array);
    (xmean, sigma)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::stats::WeightMode;
    #[test]
    fn test_xfit() {
        let small_vec: Vec<f64> = vec![1., 2., 3.];
        assert_eq!(xfit(&small_vec), (2., 1.));
    }

    #[test]
    fn test_xfit_weighted() {
        let x: Vec<f64> = vec![1., 2., 3.];
        let weights: Vec<f64> = vec![0.25, 0.5, 0.333];
        let mode: WeightMode = WeightMode::Instrumental;
        let array: StatsArray1D = StatsArray1D::new_weighted(x, weights, mode).expect("");
        assert_eq!(
            xfit_weighted(array),
            (1.7593918788730116, 0.8512102763823557)
        );
    }
}
