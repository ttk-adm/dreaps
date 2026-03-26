#[cfg(test)]
mod tests {
    use crate::math::stats::WeightMode;
    use crate::math::stats_data::StatsData;

    #[test]
    fn test_new() {
        let x: Vec<f64> = vec![1., 2., 3.];
        let y: Vec<f64> = vec![5., 5., 5.];
        let data: StatsData = StatsData::new(x, y);
        assert_eq!(data.x.array, vec![1., 2., 3.]);
        assert_eq!(data.x.weights, vec![1., 1., 1.]);
        assert_eq!(data.x.mode, WeightMode::None);
        assert_eq!(data.y.array, vec![5., 5., 5.]);
        assert_eq!(data.y.weights, vec![1., 1., 1.]);
        assert_eq!(data.y.mode, WeightMode::None);
    }

    #[test]
    #[should_panic]
    fn test_new_panic() {
        let x: Vec<f64> = vec![1., 2., 3.];
        let y: Vec<f64> = vec![5., 5.];
        StatsData::new(x, y);
    }

    #[test]
    fn test_new_statistical() {
        let x: Vec<f64> = vec![1., 2., 3.];
        let y: Vec<f64> = vec![5., 5., 5.];
        // let weights = vec![0.1, 0.2, 0.3];
        let data: StatsData = StatsData::new_statistical_weights(x, y);
        assert_eq!(data.x.array, vec![1., 2., 3.]);
        assert_eq!(data.x.weights, vec![1., 1., 1.]);
        assert_eq!(data.x.mode, WeightMode::None);
        assert_eq!(data.y.array, vec![5., 5., 5.]);
        assert_eq!(data.y.weights, vec![0.2, 0.2, 0.2]);
        assert_eq!(data.y.mode, WeightMode::Statistical);
    }

    #[test]
    #[should_panic]
    fn test_new_statistical_panic() {
        let x: Vec<f64> = vec![1., 2., 3.];
        let y: Vec<f64> = vec![5., 5.];
        StatsData::new_statistical_weights(x, y);
    }

    #[test]
    fn test_new_weighted() {
        let x: Vec<f64> = vec![1., 2., 3.];
        let y: Vec<f64> = vec![5., 5., 5.];
        let weights: Vec<f64> = vec![0.1, 0.2, 0.3];
        let mode: WeightMode = WeightMode::Instrumental;
        let data: StatsData = StatsData::new_weighted(x, y, weights, mode);
        assert_eq!(data.x.array, vec![1., 2., 3.]);
        assert_eq!(data.x.weights, vec![1., 1., 1.]);
        assert_eq!(data.x.mode, WeightMode::None);
        assert_eq!(data.y.array, vec![5., 5., 5.]);
        assert_eq!(data.y.weights, vec![0.1, 0.2, 0.3]);
        assert_eq!(data.y.mode, WeightMode::Instrumental);
    }

    #[test]
    #[should_panic]
    fn test_new_weighted_panic() {
        let x: Vec<f64> = vec![1., 2., 3.];
        let y: Vec<f64> = vec![5., 5.];
        StatsData::new_weighted(x, y, vec![1.], WeightMode::None);
    }

    #[test]
    fn test_sum_of_products() {
        let x: Vec<f64> = vec![1., 2., 3.];
        let y: Vec<f64> = vec![5., 5., 5.];
        let data: StatsData = StatsData::new(x, y);
        assert_eq!(data.sum_of_products(), 30.);
        let weights: Vec<f64> = vec![0.1, 0.2, 0.3];
        let data =
            StatsData::new_weighted(data.x.array, data.y.array, weights, WeightMode::Statistical);
        assert_eq!(data.sum_of_products(), 150.);
        let data = StatsData::new_weighted(
            data.x.array,
            data.y.array,
            data.y.weights,
            WeightMode::Instrumental,
        );
        assert_eq!(data.sum_of_products(), 916.6666666666665);
    }

    #[test]
    fn test_push() {
        let x: Vec<f64> = vec![1., 2., 3.];
        let y: Vec<f64> = vec![5., 5., 5.];
        let weights: Vec<f64> = vec![0.1, 0.2, 0.3];
        let mode: WeightMode = WeightMode::Instrumental;
        let mut data: StatsData = StatsData::new_weighted(x, y, weights, mode);
        data.push(4., 51., 2.);
        assert_eq!(data.x.array, vec![1., 2., 3., 4.]);
        assert_eq!(data.x.weights, vec![1., 1., 1., 1.]);
        assert_eq!(data.y.array, vec![5., 5., 5., 51.]);
        assert_eq!(data.y.weights, vec![0.1, 0.2, 0.3, 2.]);
    }

    #[test]
    fn test_len() {
        let x: Vec<f64> = vec![1., 2., 3.];
        let y: Vec<f64> = vec![5., 5., 5.];
        let weights: Vec<f64> = vec![0.1, 0.2, 0.3];
        let mode: WeightMode = WeightMode::Instrumental;
        let data: StatsData = StatsData::new_weighted(x, y, weights, mode);
        assert_eq!(data.len(), 3);
    }
}
