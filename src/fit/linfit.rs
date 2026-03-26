use crate::math::stats_data::StatsData;

pub struct LinFitStats {
    slope: f64,
    intercept: f64,
    slope_error: f64,
    intercept_error: f64,
    r: f64,
}

impl LinFitStats {
    pub fn print(&self) {
        println!(
            "slope: {} ± {}\nintercept: {} ± {}\nr: {}",
            self.slope, self.slope_error, self.intercept, self.intercept_error, self.r
        );
    }
}

pub fn linfit(data: StatsData) -> LinFitStats {
    LinFitStats {
        slope: data.x.mean(),
        intercept: data.y.mean(),
        slope_error: data.x.stdev(),
        intercept_error: data.y.stdev(),
        r: 0.987,
    }
    // let sum = data.y.wsum();
    // let sumx = data.x.sum();
    // let iter_x = x.iter();
    // let iter_y = y.iter();
}
