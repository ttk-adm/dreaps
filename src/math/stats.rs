#[derive(Copy, Clone, Debug, PartialEq)]
pub enum WeightMode {
    None,
    Instrumental,
    Statistical,
}

pub fn square_diff(_a: f64, _b: f64) -> f64 {
    (_a - _b).powi(2)
}
