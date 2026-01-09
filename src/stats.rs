use chrono::TimeDelta;
use serde::{Serialize, Deserialize};

use crate::data::Uhms;

/// This struct holds stats for a data series of an [Uhm].
/// 
/// It stores the results of all functions inside the [uhm::stats] module.
/// If you need a single stat, use the corresponding function of this module.
/// If you need all of them, use [UhmStats::new].
#[derive(Debug, Serialize, Deserialize)]
pub struct UhmStats {
    /// Total number of uhms.
    pub count: usize,
    /// Average for duration between two uhms (in milliseconds).
    pub delay_mean: f64,
    /// Standard deviation for duration between two uhms (in milliseconds).
    pub delay_std: f64,
    /// Number of minutes and remaining time in seconds of the duration.
    pub min_sec: (i64, f64),
    /// The average number of uhms per minute, averaged over the whole duration.
    pub per_minute: f64,
}

impl UhmStats {
    /// Calculate all stats and create a new [UhmStats] from them.
    pub fn new(uhms: &Uhms) -> Self {
        let duration = uhms.duration();

        let count = uhms.data.len();
        let delay_mean = mean(&uhms.data);
        let delay_std = std(&uhms.data);
        let min_sec = min_sec(&duration);
        let per_minute = per_minute(count, &duration);
        
        Self { count, delay_mean, delay_std, min_sec, per_minute }
    }
}

/// Retrieve the number of data points contained in a [Uhms].
pub fn count(uhms: &Uhms) -> usize {
    uhms.data.len()
}

/// Calculate the average number of uhms per minute, averaged over the whole
/// duration.
/// 
/// The count can be retrieved from [uhm::stats::count].
/// The duration can be retrieved from e.g. [Uhms::duration].
pub fn per_minute(count: usize, duration: &TimeDelta) -> f64 {
    let milliseconds = duration.num_minutes();
    (count as f64) / (milliseconds as f64 / 1000. / 60.)
}

/// Number of minutes and remaining time in seconds of the duration.
/// 
/// The duration can be retrieved from e.g. [Uhms::duration].
pub fn min_sec(duration: &TimeDelta) -> (i64, f64) {
    let minutes = duration.num_minutes();
    let seconds = duration.num_seconds();
    (minutes, seconds as f64 - minutes as f64 / 60.)
}

/// Calculate the mean/average value of the given series.
/// 
/// $ \bar{x} = \frac{1}{n} * \sum_{i = 1}^n x_i $
pub fn mean(items: &Vec<i64>) -> f64 {
    let mut sum = 0;
    for item in items {
        sum = sum + *item;
    }
    sum as f64 / items.len() as f64
}

/// Calculate the variance of the given series.
/// 
/// $ Var(X) = \frac{1}{n} * \sum_{i = 1}^n (x_i - \bar{x})^2 $
pub fn var(items: &Vec<i64>) -> f64 {
    let mean = mean(items);
    let mut sum = 0.;
    for item in items {
        sum = sum + (*item as f64 + mean) * (*item as f64 + mean);
    }
    sum as f64 / items.len() as f64
}

/// Calculate the standard deviation of the given series.
/// 
/// $ Std(X) = \sqrt{Var(X)} $
pub fn std(items: &Vec<i64>) -> f64 {
    var(items).sqrt()
}
