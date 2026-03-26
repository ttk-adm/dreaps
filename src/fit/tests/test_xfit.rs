#[cfg(test)]
mod tests {
    use crate::fit::xfit::xfit;
    use crate::math::stats::WeightMode;
    use crate::math::stats_array::StatsArray;

    #[test]
    fn test_xfit() {
        let x: Vec<f64> = vec![1., 2., 3.];
        let array = StatsArray::new(x);
        assert_eq!(xfit(array), (2., 1.));
    }

    #[test]
    fn test_xfit_weighted() {
        let x: Vec<f64> = vec![1., 2., 3.];
        let weights: Vec<f64> = vec![0.25, 0.5, 0.333];
        let mode: WeightMode = WeightMode::Instrumental;
        let array: StatsArray = StatsArray::new_weighted(x, weights, mode);
        assert_eq!(xfit(array), (1.7593918788730116, 1.042515420475107));
    }
}
