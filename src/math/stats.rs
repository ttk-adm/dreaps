use std::iter::Zip;
use std::slice::Iter;

#[derive(Copy, Clone)]
pub enum WeightMode {
    None,
    Instrumental,
    Statistical,
}

pub fn compare_len(array_a: &[f64], array_b: &[f64]) -> Result<(), String> {
    if array_a.len() != array_b.len() {
        Err("arrays must have equal lengths".into())
    } else {
        Ok(())
    }
}

#[derive(Clone)]
pub struct StatsArray1D {
    pub array: Vec<f64>,
    pub weights: Vec<f64>,
    pub mode: WeightMode,
}

impl StatsArray1D {
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
    pub fn zip<'a, I>(&'a self, other: I) -> Zip<Iter<'a, f64>, I::IntoIter>
    where
        I: IntoIterator<Item = &'a f64>,
    {
        self.iter().zip(other)
    }

    #[inline]
    pub fn wzip(&self) -> Zip<Iter<'_, f64>, Iter<'_, f64>> {
        self.zip(self.witer())
    }

    #[inline]
    pub fn map<'a, F>(&'a self, f: F) -> impl Iterator<Item = f64> + 'a
    where
        F: FnMut(f64) -> f64 + 'a,
    {
        self.iter().copied().map(f)
    }

    #[inline]
    pub fn wmap<'a, F>(&'a self, f: F) -> impl Iterator<Item = f64> + 'a
    where
        F: FnMut(f64) -> f64 + 'a,
    {
        self.weights.iter().copied().map(f)
    }

    pub fn sum(&self) -> f64 {
        let order: i32 = match self.mode {
            WeightMode::None => return self.array.iter().sum(),
            WeightMode::Instrumental => -2,
            WeightMode::Statistical => -1,
        };
        self.wzip().map(|(x, w)| x * w.powi(order)).sum()
    }

    pub fn mean(&self) -> f64 {
        let order: i32 = match self.mode {
            WeightMode::None => {
                return {
                    let sum: f64 = self.array.iter().sum();
                    sum / self.len() as f64
                };
            }
            WeightMode::Instrumental => -2,
            WeightMode::Statistical => 0,
        };
        let weighting = self.weights.iter().map(|weight| weight.powi(order));
        let sum_weights: f64 = weighting.clone().sum();
        let sum: f64 = weighting.zip(self.iter()).map(|(w, x)| w * x).sum();
        sum / sum_weights
    }

    pub fn stdev(&self) -> f64 {
        let mean: f64 = self.mean();
        let dof: f64 = self.len() as f64 - 1.0;
        let variance: f64 = self
            .map(|value: f64| {
                let diff: f64 = mean - value;
                diff * diff
            })
            .sum::<f64>()
            / dof;
        variance.sqrt()
    }

    pub fn sum_of_squares(&self) -> f64 {
        let order: i32 = match self.mode {
            WeightMode::None => return self.array.iter().map(|x: &f64| x * x).sum(),
            WeightMode::Instrumental => -2,
            WeightMode::Statistical => -1,
        };
        self.wzip().map(|(x, w)| x * x * w.powi(order)).sum()
    }
}

#[derive(Clone)]
pub struct StatsArray2D {
    pub x: StatsArray1D,
    pub y: StatsArray1D,
}

impl StatsArray2D {
    pub fn new(x: Vec<f64>, y: Vec<f64>) -> Self {
        assert_eq!(x.len(), y.len());
        Self {
            x: StatsArray1D::new(x),
            y: StatsArray1D::new(y),
        }
    }

    pub fn new_statistical_weights(x: Vec<f64>, y: Vec<f64>) -> Self {
        assert_eq!(x.len(), y.len());
        let weights: Vec<f64> = y.iter().map(|_y: &f64| _y.powi(-1)).collect();
        let mode: WeightMode = WeightMode::Statistical;
        Self {
            x: StatsArray1D::new_weighted(x, weights.clone(), mode),
            y: StatsArray1D::new_weighted(y, weights, mode),
        }
    }

    pub fn new_weighted(x: Vec<f64>, y: Vec<f64>, weights: Vec<f64>, mode: WeightMode) -> Self {
        assert_eq!(x.len(), y.len());
        let status: Result<(), String> = compare_len(&x, &weights);
        match status {
            Err(_) => Self::new_statistical_weights(x, y),
            Ok(_) => Self {
                x: StatsArray1D::new_weighted(x, weights.clone(), mode),
                y: StatsArray1D::new_weighted(y, weights, mode),
            },
        }
    }

    pub fn zipxy(&self) -> Zip<std::slice::Iter<'_, f64>, std::slice::Iter<'_, f64>> {
        self.x.iter().zip(self.y.iter())
    }

    pub fn wzipxy(
        &self,
    ) -> Zip<Zip<std::slice::Iter<'_, f64>, std::slice::Iter<'_, f64>>, std::slice::Iter<'_, f64>>
    {
        self.zipxy().zip(self.x.weights.iter())
    }

    pub fn sum_of_products(&self) -> f64 {
        let order: i32 = match self.x.mode {
            WeightMode::None => return self.x.iter().zip(self.y.iter()).map(|(x, y)| x * y).sum(),
            WeightMode::Instrumental => -2,
            WeightMode::Statistical => -1,
        };
        self.wzipxy().map(|((x, y), w)| x * y * w.powi(order)).sum()
    }

    pub fn push(&mut self, x: f64, y: f64, weight: f64) {
        self.x.push(x, weight);
        self.y.push(y, weight);
    }

    pub fn len(&self) -> usize {
        self.x.len()
    }
}
