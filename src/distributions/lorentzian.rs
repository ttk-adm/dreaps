pub struct LorentzianDistribution {}

impl LorentzianDistribution {
    pub fn coefficient(&self, x: f64, mean: f64, fwhm: f64) -> f64 {
        use std::f64::consts::PI;
        let hwhm: f64 = fwhm / 2.0;
        hwhm / (PI * ((x - mean).powi(2) + hwhm.powi(2)))
    }
}

#[test]
fn test_coefficient() {
    let lorentzdist = LorentzianDistribution {};
    let lorentzcoeff = lorentzdist.coefficient(0.0, 0.0, 1.0);
    println!("Binomial Coefficient: {lorentzcoeff}");
}
