use crate::math::stats_data::StatsData;

pub struct LinFitStats {
    pub slope: f64,
    pub intercept: f64,
    pub slope_error: f64,
    pub intercept_error: f64,
    pub r2: f64,
}

impl LinFitStats {
    pub fn print(&self) {
        println!(
            "slope: {} ± {}\nintercept: {} ± {}\nr: {}",
            self.slope, self.slope_error, self.intercept, self.intercept_error, self.r2
        );
    }
}

pub fn linfit(data: &StatsData) -> LinFitStats {
    let sumw: f64 = data.y.wsum();
    let sumx: f64 = data.x.sum();
    let sumy: f64 = data.y.sum();
    let sumx2: f64 = data.x.sum_of_squares();
    let sumy2: f64 = data.y.sum_of_squares();
    let sumxy: f64 = data.sum_of_products();

    let delta: f64 = (sumw * sumx2) - (sumx * sumx);
    let intercept: f64 = (sumx2 * sumy - sumx * sumxy) / delta;
    let slope: f64 = (sumxy * sumw - sumx * sumy) / delta;
    let variance: f64 = {
        let n: f64 = data.lenf64() - 2.0;

        let intercept_term: f64 = intercept.powi(2) * sumw;
        let slope_term: f64 = slope.powi(2) * sumx2;

        let cross_intercept_sumy: f64 = intercept * sumy;
        let cross_slope_sumxy: f64 = slope * sumxy;
        let cross_slope_intercept_sumx: f64 = slope * intercept * sumx;

        let adjustment: f64 =
            2.0 * (cross_intercept_sumy + cross_slope_sumxy - cross_slope_intercept_sumx);

        let numerator: f64 = sumy2 + intercept_term + slope_term - adjustment;

        numerator / n
    };
    let slope_error: f64 = (variance * sumw / delta).sqrt();
    let intercept_error: f64 = (variance * sumx2 / delta).sqrt();
    let r: f64 = (sumw * sumxy - sumx * sumy) / (delta * (sumw * sumy2 - sumy * sumy)).sqrt();

    LinFitStats {
        slope: slope,
        intercept: intercept,
        slope_error: slope_error,
        intercept_error: intercept_error,
        r2: r,
    }
}
