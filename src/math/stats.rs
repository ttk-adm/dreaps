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

pub fn mean_weighted(narray: &[f64], weights: &[f64]) -> f64 {
    let weighting = weights.iter().map(|weight: &f64| 1.0 / (weight * weight));
    let sum_weights: f64 = weighting.clone().sum();
    let sum: f64 = weighting.zip(narray.iter()).map(|(w, n)| w * n).sum();
    sum / sum_weights
}

pub fn stdev_weighted(narray: &[f64], weights: &[f64]) -> f64 {
    let n_mean: f64 = mean_weighted(narray, weights);
    let variance: f64 = narray
        .iter()
        .map(|value: &f64| {
            let diff: f64 = n_mean - value;
            diff * diff
        })
        .sum::<f64>()
        / narray.len() as f64;
    variance.sqrt()
}
