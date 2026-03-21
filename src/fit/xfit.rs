use crate::math::stats::StatsArray1D;

pub fn xfit(array: StatsArray1D) -> (f64, f64) {
    (array.mean(), array.stdev())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::stats::WeightMode;
    #[test]
    fn test_xfit() {
        let x: Vec<f64> = vec![1., 2., 3.];
        let array = StatsArray1D::new(x);
        assert_eq!(xfit(array), (2., 1.));
    }

    #[test]
    fn test_xfit_weighted() {
        let x: Vec<f64> = vec![1., 2., 3.];
        let weights: Vec<f64> = vec![0.25, 0.5, 0.333];
        let mode: WeightMode = WeightMode::Instrumental;
        let array: StatsArray1D = StatsArray1D::new_weighted(x, weights, mode);
        assert_eq!(xfit(array), (1.7593918788730116, 1.042515420475107));
    }
}
