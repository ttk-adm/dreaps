#[cfg(test)]
mod tests {
    use crate::fit::linfit::{linfit, LinFitStats};
    use crate::math::stats::WeightMode;
    use crate::math::stats_data::StatsData;

    #[test]
    fn test_linfit_exact() {
        let x: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let data: StatsData = StatsData::new(x.clone(), x);
        let fit: LinFitStats = linfit(&data);
        assert_eq!(fit.slope, 1.);
        assert_eq!(fit.intercept, 0.);
        assert_eq!(fit.slope_error, 0.);
        assert_eq!(fit.intercept_error, 0.);
        assert_eq!(fit.r, 1.);

        let y: Vec<f64> = vec![1., 3., 5., 7., 9.];
        let data: StatsData = StatsData::new(data.x.array, y);
        let fit: LinFitStats = linfit(&data);
        assert_eq!(fit.slope, 2.);
        assert_eq!(fit.intercept, -1.);
        assert_eq!(fit.slope_error, 0.);
        assert_eq!(fit.intercept_error, 0.);
        assert_eq!(fit.r, 1.);

        let y: Vec<f64> = vec![-13., -5., 3., 11., 19.];
        let data: StatsData = StatsData::new(data.x.array, y);
        let fit: LinFitStats = linfit(&data);
        assert_eq!(fit.slope, 8.);
        assert_eq!(fit.intercept, -21.);
        assert_eq!(fit.slope_error, 0.);
        assert_eq!(fit.intercept_error, 0.);
        assert_eq!(fit.r, 1.);
        println!("{}", fit.intercept)
    }

    #[test]
    fn test_linfit_simple() {
        let x: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let y: Vec<f64> = vec![0.9, 2.1, 3.2, 3.8, 5.5];
        let data: StatsData = StatsData::new(x, y);
        let fit: LinFitStats = linfit(&data);
        assert_eq!(fit.slope, 1.09);
        assert_eq!(fit.intercept, -0.17000000000000226);
        assert_eq!(fit.slope_error, 0.08544003745317416);
        assert_eq!(fit.intercept_error, 0.28337254630609127);
        assert_eq!(fit.r, 0.990909090909091);
    }

    #[test]
    fn test_linfit_weights() {
        let x: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let y: Vec<f64> = vec![0.9, 2.1, 3.2, 3.8, 5.5];
        let weights: Vec<f64> = vec![0.11, 0.12, 0.13, 0.14, 0.15];
        let mode: WeightMode = WeightMode::Statistical;
        let data: StatsData = StatsData::new_weighted(x, y, weights, mode);
        let fit: LinFitStats = linfit(&data);
        println!("{}", fit.slope);
        println!("{}", fit.intercept);
        println!("{}", fit.slope_error);
        println!("{}", fit.intercept_error);
        println!("{}", fit.r);
    }
}