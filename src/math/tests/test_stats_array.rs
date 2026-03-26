#[cfg(test)]
mod tests {
    // use super::*;
    use crate::math::stats::WeightMode;
    use crate::math::stats_array::StatsArray;

    #[test]
    fn test_new() {
        let array: Vec<f64> = vec![1., 2., 3.];
        let stat_array: StatsArray = StatsArray::new(array);
        assert_eq!(stat_array.array, vec![1., 2., 3.]);
        assert_eq!(stat_array.weights, vec![1., 1., 1.]);
        assert_eq!(stat_array.mode, WeightMode::None);
    }

    #[test]
    fn test_new_weighted() {
        let array: Vec<f64> = vec![1., 2., 3.];
        let weights: Vec<f64> = vec![0.1, 0.2, 0.3];
        let mode: WeightMode = WeightMode::Statistical;
        let stat_array: StatsArray = StatsArray::new_weighted(array, weights, mode);
        assert_eq!(stat_array.array, vec![1., 2., 3.]);
        assert_eq!(stat_array.weights, vec![10., 5., 3.3333333333333335]);
        assert_eq!(stat_array.mode, WeightMode::Statistical);

        let array: Vec<f64> = vec![1., 2., 3.];
        let weights: Vec<f64> = vec![0.1, 0.2, 0.3];
        let mode: WeightMode = WeightMode::Instrumental;
        let stat_array: StatsArray = StatsArray::new_weighted(array, weights, mode);
        assert_eq!(stat_array.array, vec![1., 2., 3.]);
        assert_eq!(stat_array.weights, vec![99.99999999999999, 24.999999999999996, 11.11111111111111]);
        assert_eq!(stat_array.mode, WeightMode::Instrumental);
    }

    #[test]
    fn test_new_weighted_mismatch() {
        let array: Vec<f64> = vec![1., 2., 3.];
        let weights: Vec<f64> = vec![0.1, 0.2];
        let mode: WeightMode = WeightMode::Instrumental;
        let stat_array: StatsArray = StatsArray::new_weighted(array, weights, mode);
        assert_eq!(stat_array.array, vec![1., 2., 3.]);
        assert_eq!(stat_array.weights, vec![1., 1., 1.]);
        assert_eq!(stat_array.mode, WeightMode::None);
    }

    #[test]
    fn test_sum() {
        let array: Vec<f64> = vec![1., 2., 3.];
        let stat_array: StatsArray = StatsArray::new(array);
        assert_eq!(stat_array.sum(), 6.);

        let array: Vec<f64> = vec![1., 2., 3.];
        let weights: Vec<f64> = vec![0.1, 0.2, 0.3];
        let mode: WeightMode = WeightMode::Instrumental;
        let w_stat_array: StatsArray = StatsArray::new_weighted(array, weights, mode);
        assert_eq!(w_stat_array.sum(), 183.33333333333331);
    }

    #[test]
    fn test_mean() {
        let array: Vec<f64> = vec![1., 2., 3.];
        let stat_array: StatsArray = StatsArray::new(array);
        assert_eq!(stat_array.mean(), 2.);

        let array: Vec<f64> = vec![1., 2., 3.];
        let weights: Vec<f64> = vec![0.1, 0.2, 0.3];
        let mode: WeightMode = WeightMode::Statistical;
        let w_stat_array: StatsArray = StatsArray::new_weighted(array, weights, mode);
        assert_eq!(w_stat_array.mean(), 2.);

        let array: Vec<f64> = vec![1., 2., 3.];
        let weights: Vec<f64> = vec![0.1, 0.2, 0.3];
        let mode: WeightMode = WeightMode::Instrumental;
        let w_stat_array: StatsArray = StatsArray::new_weighted(array, weights, mode);
        assert_eq!(w_stat_array.mean(), 1.3469387755102042);
    }

    #[test]
    fn test_stdev() {
        let array: Vec<f64> = vec![1., 2., 3.];
        let stat_array: StatsArray = StatsArray::new(array);
        assert_eq!(stat_array.stdev(), 1.);

        let array: Vec<f64> = vec![1., 2., 3.];
        let weights: Vec<f64> = vec![0.1, 0.2, 0.3];
        let mode: WeightMode = WeightMode::Statistical;
        let w_stat_array: StatsArray = StatsArray::new_weighted(array, weights, mode);
        assert_eq!(w_stat_array.stdev(), 1.);

        let array: Vec<f64> = vec![1., 2., 3.];
        let weights: Vec<f64> = vec![0.1, 0.2, 0.3];
        let mode: WeightMode = WeightMode::Instrumental;
        let w_stat_array: StatsArray = StatsArray::new_weighted(array, weights, mode);
        assert_eq!(w_stat_array.stdev(), 1.2805207707796726);
    }

    #[test]
    fn test_sum_of_squares() {
        let array: Vec<f64> = vec![1., 2., 3.];
        let stat_array: StatsArray = StatsArray::new(array);
        assert_eq!(stat_array.sum_of_squares(), 14.);

        let array: Vec<f64> = vec![1., 2., 3.];
        let weights: Vec<f64> = vec![0.1, 0.2, 0.3];
        let mode: WeightMode = WeightMode::Statistical;
        let w_stat_array: StatsArray = StatsArray::new_weighted(array, weights, mode);
        assert_eq!(w_stat_array.sum_of_squares(), 60.);

        let array: Vec<f64> = vec![1., 2., 3.];
        let weights: Vec<f64> = vec![0.1, 0.2, 0.3];
        let mode: WeightMode = WeightMode::Instrumental;
        let w_stat_array: StatsArray = StatsArray::new_weighted(array, weights, mode);
        assert_eq!(w_stat_array.sum_of_squares(), 300.);
    }
}
