#[derive(Copy, Clone)]
pub enum WeightMode {
    None,
    Instrumental,
    Statistical,
}

#[derive(Clone)]
pub struct StatsArray1D {
    x: Vec<f64>,
    weights: Vec<f64>,
    mode: WeightMode,
}

impl StatsArray1D {
    pub fn new(x: Vec<f64>) -> Self {
        let weights: Vec<f64> = x.iter().map(|_| 1.0).collect();
        Self {
            x,
            weights,
            mode: WeightMode::None,
        }
    }

    pub fn new_weighted(x: Vec<f64>, weights: Vec<f64>, mode: WeightMode) -> Result<Self, String> {
        if x.len() != weights.len() {
            return Err("x and weights must have equal lengths".into());
        }
        Ok(Self { x, weights, mode })
    }

    pub fn push(&mut self, x: f64, weight: f64) {
        self.x.push(x);
        self.weights.push(weight);
    }

    pub fn len(&self) -> usize {
        self.x.len()
    }
}

#[derive(Clone)]
pub struct StatsArray2D {
    x: Vec<f64>,
    y: Vec<f64>,
    weights: Vec<f64>,
    mode: WeightMode,
}

impl StatsArray2D {
    pub fn new(x: Vec<f64>, y: Vec<f64>) -> Result<Self, String> {
        if x.len() != y.len() {
            return Err("x and y must have same lengths".into());
        }
        let weights: Vec<f64> = x.iter().map(|_| 1.0).collect();
        let mode: WeightMode = WeightMode::None;
        Ok(Self {
            x,
            y,
            weights,
            mode,
        })
    }

    pub fn new_statistical_weights(x: Vec<f64>, y: Vec<f64>) -> Result<Self, String> {
        if x.len() != y.len() {
            return Err("x and y must have same lengths".into());
        }
        let weights: Vec<f64> = y.iter().map(|_y: &f64| _y.powi(-1)).collect();
        let mode: WeightMode = WeightMode::Statistical;
        Ok(Self {
            x,
            y,
            weights,
            mode,
        })
    }

    pub fn new_weighted(
        x: Vec<f64>,
        y: Vec<f64>,
        weights: Vec<f64>,
        mode: WeightMode,
    ) -> Result<Self, String> {
        if x.len() != y.len() || x.len() != weights.len() {
            return Err("x, y, and weights must have same lengths".into());
        }
        Ok(Self {
            x,
            y,
            weights,
            mode,
        })
    }
}

// pub fn weighted_sum(narray: &[f64], weights: &[f64], mode: WeightMode) -> f64 {
//     let order: i32 = match mode {
//         WeightMode::Instrumental => -2,
//         WeightMode::Statistical => -1,
//     };
//     narray
//         .iter()
//         .zip(weights.iter())
//         .map(|(n, w)| n * w.powi(order))
//         .sum()
// }

pub fn weighted_sum(array: StatsArray1D) -> f64 {
    let order: i32 = match array.mode {
        WeightMode::None => return array.x.iter().sum(),
        WeightMode::Instrumental => -2,
        WeightMode::Statistical => -1,
    };
    array
        .x
        .iter()
        .zip(array.weights.iter())
        .map(|(x, w)| x * w.powi(order))
        .sum()
}

pub fn mean(narray: &[f64]) -> f64 {
    let sum: f64 = narray.iter().sum();
    sum / narray.len() as f64
}

pub fn stdev(narray: &[f64]) -> f64 {
    let n_mean: f64 = mean(narray);
    let dof: f64 = narray.len() as f64 - 1.;
    let variance: f64 = narray
        .iter()
        .map(|value: &f64| {
            let diff: f64 = n_mean - value;
            diff * diff
        })
        .sum::<f64>()
        / dof;
    variance.sqrt()
}

// pub fn mean_weighted(narray: &[f64], weights: &[f64], mode: WeightMode) -> f64 {
//     let order: i32 = match mode {
//         WeightMode::Instrumental => -2,
//         WeightMode::Statistical => 0,
//     };
//     let weighting = weights.iter().map(|weight: &f64| weight.powi(order));
//     let sum_weights: f64 = weighting.clone().sum();
//     let sum: f64 = weighting.zip(narray.iter()).map(|(w, n)| w * n).sum();
//     sum / sum_weights
// }

pub fn mean_weighted(array: StatsArray1D) -> f64 {
    let order: i32 = match array.mode {
        WeightMode::None => {
            return {
                let sum: f64 = array.x.iter().sum();
                sum / array.x.len() as f64
            };
        }
        WeightMode::Instrumental => -2,
        WeightMode::Statistical => 0,
    };
    let weighting = array.weights.iter().map(|weight: &f64| weight.powi(order));
    let sum_weights: f64 = weighting.clone().sum();
    let sum: f64 = weighting.zip(array.x.iter()).map(|(w, x)| w * x).sum();
    sum / sum_weights
}

// pub fn stdev_weighted(narray: &[f64], weights: &[f64], mode: WeightMode) -> f64 {
//     let n_mean: f64 = mean_weighted(narray, weights, mode);
//     let variance: f64 = narray
//         .iter()
//         .map(|value: &f64| {
//             let diff: f64 = n_mean - value;
//             diff * diff
//         })
//         .sum::<f64>()
//         / narray.len() as f64;
//     variance.sqrt()
// }

pub fn stdev_weighted(array: StatsArray1D) -> f64 {
    let x_mean: f64 = mean_weighted(array.clone());
    let variance: f64 = array
        .x
        .iter()
        .map(|value: &f64| {
            let diff: f64 = x_mean - value;
            diff * diff
        })
        .sum::<f64>()
        / array.x.len() as f64;
    variance.sqrt()
}

pub fn weighted_sum_of_squares(array: StatsArray1D) -> f64 {
    let order: i32 = match array.mode {
        WeightMode::None => return array.x.iter().map(|x: &f64| x * x).sum(),
        WeightMode::Instrumental => -2,
        WeightMode::Statistical => -1,
    };
    array
        .x
        .iter()
        .zip(array.weights.iter())
        .map(|(x, w)| x * x * w.powi(order))
        .sum()
}

pub fn weighted_sum_of_products(array: StatsArray2D) -> f64 {
    let order: i32 = match array.mode {
        WeightMode::None => return array.x.iter().zip(array.y.iter()).map(|(x, y)| x * y).sum(),
        WeightMode::Instrumental => -2,
        WeightMode::Statistical => -1,
    };
    array
        .x
        .iter()
        .zip(array.y.iter())
        .zip(array.weights.iter())
        .map(|((x, y), w)| x * y * w.powi(order))
        .sum()
}
