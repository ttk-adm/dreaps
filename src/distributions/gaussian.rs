pub struct GaussianDistribution {}

impl GaussianDistribution {
    pub fn coefficient(&self, x: f64, mean: f64, sigma: f64) -> f64 {
        use std::f64::consts::PI;
        let z = (x - mean) / sigma;
        1.0 / (sigma * (2.0 * PI).sqrt()) * (-1.0 / 2.0 * z.powi(2)).exp()
    }
}

#[test]
fn test_coefficient() {
    let gaussdist = GaussianDistribution {};
    let gausscoeff = gaussdist.coefficient(0.0, 0.0, 1.0);
    println!("Binomial Coefficient: {gausscoeff}");
}
