use crate::math::array::compare_len;
use crate::math::stats::WeightMode;
use std::iter::Zip;
use std::slice::Iter;

#[derive(Clone)]
pub struct StatsArray {
    pub array: Vec<f64>,
    pub weights: Vec<f64>,
    pub mode: WeightMode,
}

impl StatsArray {
    pub fn new(array: Vec<f64>) -> Self {
        let weights: Vec<f64> = vec![1.0; array.len()];
        Self {
            array,
            weights,
            mode: WeightMode::None,
        }
    }

    pub fn new_weighted(array: Vec<f64>, weights: Vec<f64>, mode: WeightMode) -> Self {
        if compare_len(&array, &weights).is_err() {
            return Self::new(array);
        }
        Self {
            array,
            weights,
            mode,
        }
    }

    pub fn push(&mut self, new_val: f64, weight: f64) {
        self.array.push(new_val);
        self.weights.push(weight);
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.array.len()
    }

    #[inline]
    pub fn lenf(&self) -> f64 {
        self.len() as f64
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.array.is_empty()
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, f64> {
        self.array.iter()
    }

    #[inline]
    pub fn witer(&self) -> Iter<'_, f64> {
        self.weights.iter()
    }

    #[inline]
    pub fn wzip(&self) -> Zip<Iter<'_, f64>, Iter<'_, f64>> {
        self.iter().zip(self.witer())
    }

    #[inline]
    pub fn dof(&self) -> f64 {
        self.lenf() - 1.0
    }

    pub fn weighting(&self) -> impl Iterator<Item = f64> {
        let order: i32 = match self.mode {
            WeightMode::None => 0,
            WeightMode::Instrumental => -2,
            WeightMode::Statistical => -1,
        };
        self.witer().map(move |_w: &f64| _w.powi(order))
    }

    pub fn sum(&self) -> f64 {
        let order: i32 = match self.mode {
            WeightMode::None => return self.iter().sum(),
            WeightMode::Instrumental => -2,
            WeightMode::Statistical => -1,
        };
        self.wzip().map(|(_n, _w)| _n * _w.powi(order)).sum()
    }

    pub fn mean(&self) -> f64 {
        let order: i32 = match self.mode {
            WeightMode::None => return self.sum() / self.lenf(),
            WeightMode::Instrumental => -2,
            WeightMode::Statistical => 0,
        };
        let sum_weights: f64 = self.witer().map(move |_w: &f64| _w.powi(order)).sum();
        let sum: f64 = self.wzip().map(|(_n, _w)| _n * _w.powi(order)).sum();
        sum / sum_weights
    }

    fn sdm(&self) -> f64 {
        use crate::math::stats::square_diff;
        self.iter().map(|_n| square_diff(self.mean(), *_n)).sum()
    }

    pub fn variance(&self) -> f64 {
        self.sdm() / self.dof()
    }

    pub fn stdev(&self) -> f64 {
        self.variance().sqrt()
    }

    pub fn sum_of_squares(&self) -> f64 {
        let order: i32 = match self.mode {
            WeightMode::None => return self.iter().map(|_n: &f64| _n * _n).sum(),
            WeightMode::Instrumental => -2,
            WeightMode::Statistical => -1,
        };
        self.wzip().map(|(_n, _w)| _n * _n * _w.powi(order)).sum()
    }
}
