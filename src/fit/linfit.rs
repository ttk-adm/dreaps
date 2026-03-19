pub struct LinFitStats {
    slope: f64,
    intercept: f64,
    slope_error: f64,
    intercept_error: f64,
    r: f64,
}

pub fn linfit(x: &[f64], y: &[f64]) -> LinFitStats {
    use crate::math::stats::{mean, stdev};
    LinFitStats {
        slope: mean(x),
        intercept: mean(y),
        slope_error: stdev(x),
        intercept_error: stdev(y),
        r: 0.987,
    }
    // let iter_x = x.iter();
    // let iter_y = y.iter();
}
