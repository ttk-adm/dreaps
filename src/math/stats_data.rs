use crate::math::array::compare_len;
use crate::math::stats::WeightMode;
use crate::math::stats_array::StatsArray;
use std::iter::Zip;

#[derive(Clone)]
pub struct StatsData {
    pub x: StatsArray,
    pub y: StatsArray,
}

impl StatsData {
    pub fn new(x: Vec<f64>, y: Vec<f64>) -> Self {
        assert_eq!(x.len(), y.len());
        Self {
            x: StatsArray::new(x),
            y: StatsArray::new(y),
        }
    }

    pub fn new_statistical_weights(x: Vec<f64>, y: Vec<f64>) -> Self {
        assert_eq!(x.len(), y.len());
        let weights: Vec<f64> = y.clone();
        let mode: WeightMode = WeightMode::Statistical;
        Self {
            x: StatsArray::new_weighted(x, weights.clone(), mode),
            y: StatsArray::new_weighted(y, weights, mode),
        }
    }

    pub fn new_weighted(x: Vec<f64>, y: Vec<f64>, weights: Vec<f64>, mode: WeightMode) -> Self {
        assert_eq!(x.len(), y.len());
        let status: Result<(), String> = compare_len(&x, &weights);
        match status {
            Err(_) => Self::new_statistical_weights(x, y),
            Ok(_) => Self {
                x: StatsArray::new_weighted(x, weights.clone(), mode),
                y: StatsArray::new_weighted(y, weights, mode),
            },
        }
    }

    fn zipxy(&self) -> Zip<std::slice::Iter<'_, f64>, std::slice::Iter<'_, f64>> {
        self.x.iter().zip(self.y.iter())
    }

    fn wzipxy(
        &self,
    ) -> Zip<Zip<std::slice::Iter<'_, f64>, std::slice::Iter<'_, f64>>, std::slice::Iter<'_, f64>>
    {
        self.zipxy().zip(self.y.weights.iter())
    }

    pub fn sum_of_products(&self) -> f64 {
        match self.y.mode {
            WeightMode::None => return self.zipxy().map(|(x, y)| x * y).sum(),
            _ => self.wzipxy().map(|((x, y), w)| x * y * w).sum()
        }
    }

    pub fn push(&mut self, x: f64, y: f64, weight: f64) {
        self.x.push(x, weight);
        self.y.push(y, weight);
    }

    pub fn len(&self) -> usize {
        self.x.len()
    }

    pub fn lenf64(&self) -> f64 {
        self.x.lenf64()
    }
}
