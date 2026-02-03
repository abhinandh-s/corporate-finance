// mean = total / count
pub fn mean(series: &[f64]) -> f64 {
    let count = series.len() as f64;
    series.iter().sum::<f64>() / count
}

// variance = for each + [ (actual - mean)^2 ] / count
//
// mean: Option<f64> => if None: function will calculate from series
pub fn variance(series: &[f64], pre_computed_mean: Option<f64>) -> f64 {
    let count = series.len() as f64;
    let m = match pre_computed_mean {
        Some(m) => m,
        None => mean(series),
    };
    series
        .iter()
        .map(|x| {
            let diff = x - m;
            diff * diff
        })
        .sum::<f64>()
        / count
}

// standard deviation = sqrt of variance
pub fn standard_deviation(variance: f64) -> f64 {
    variance.sqrt()
}
