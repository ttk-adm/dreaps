use crate::math::stats_array::StatsArray;

pub fn xfit(array: StatsArray) -> (f64, f64) {
    (array.mean(), array.stdev())
}
