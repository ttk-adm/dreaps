use crate::math::discrete::binomial_permutations;

pub struct BinomialDistribution {}

impl BinomialDistribution {
    pub fn coefficient(&self, n_obs: u64, n_total: u64, prob: f64) -> f64 {
        let not_obs: u64 = n_total - n_obs;
        let perms: f64 = binomial_permutations(n_total, n_obs);
        let coeff: f64 = perms * prob.powi(n_obs as i32) * (1.0 - prob).powi(not_obs as i32);
        coeff
    }
}

#[test]
fn test_binomial_coefficient() {
    let bidist = BinomialDistribution {};
    let bicoeff = bidist.coefficient(5, 10, 0.5);
    println!("Binomial Coefficient: {bicoeff}");
}
