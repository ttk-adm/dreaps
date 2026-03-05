use crate::math::fact_f64;

pub struct PoissonDistribution {}

impl PoissonDistribution {
    pub fn coefficient(&self, n_obs: u64, mean: f64) -> f64 {
        let fact_obs: f64 = fact_f64(n_obs);
        let coeff: f64 = (mean.powi(n_obs as i32) / fact_obs as f64) * f64::exp(-mean);
        coeff
    }
}

#[test]
fn test_poisson_coefficient() {
    let poidist = PoissonDistribution {};
    let bicoeff = poidist.coefficient(2, 1.67);
    println!("Poisson Coefficient: {bicoeff}");
}
