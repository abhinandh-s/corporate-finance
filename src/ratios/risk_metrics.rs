//! Sharpe: Is it efficient overall?
//! Sortino: Is it safe from losses?
//! Treynor: Is it better than just buying the market index?
//! Information Ratio: Is the manager actually skillful?

use crate::prelude::*;

//     Beta: Sensitivity of the portfolio to market movements.
pub struct Beta(f64);

impl Beta {
    pub fn new(series: &[f64], market: &[f64]) -> Self {
        let m_mean = mean(market);
        let s_mean = mean(series);

        // Covariance(series, market)
        let co_variance = series
            .iter()
            .zip(market.iter())
            .map(|(&s, &m)| (s - s_mean) * (m - m_mean))
            .sum::<f64>()
            / series.len() as f64;

        // Variance(market)
        let m_variance = variance(market, Some(m_mean));

        Self(co_variance / m_variance)
    }

    // if Beta > 1.0 => The fund is more volatile than the market

    /// The fund moves exactly with the market.
    pub const fn is_one(self) -> bool {
        self.0 == 1.00
    }

    /// The fund is "defensive" and moves less than the market
    pub const fn is_negative(self) -> bool {
        self.0.is_sign_negative()
    }
}

impl From<f64> for Beta {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl From<Beta> for f64 {
    fn from(value: Beta) -> Self {
        value.0
    }
}

pub struct Remarks;

impl Remarks {
    pub const EXCELLENT: &str = "Excellent";
    pub const GREAT: &str = "Great";
    pub const ACCEPTABLE: &str = "Acceptable";
    pub const BAD: &str = "Bad";
}


// # Sharpe Ratio
//
// The golden industry standard for risk-adjusted return
//
// ## What is the Sharpe Ratio?
//
// Named after American economist, William Sharpe,
// the Sharpe Ratio (or Sharpe Index or Modified Sharpe Ratio)
// is commonly used to gauge the performance of an investment
// by adjusting for its risk.
//
// The higher the ratio, the greater the investment return relative
// to the amount of risk taken, and thus, the better the investment.
// The ratio can be used to evaluate a single stock or investment, or
// an entire portfolio.
//
// ## Sharpe Ratio Formula
//
// Sharpe Ratio = (Rx – Rf) / StdDev Rx
//
// Where:
//
//     Rx = Expected portfolio return
//     Rf = Risk-free rate of return
//     StdDev Rx = Standard deviation of portfolio return (or, volatility)
//
// Sharpe Ratio Grading Thresholds:
//
//     Less than 1: Bad
//     1 – 1.99: Adequate/good
//     2 – 2.99: Very good
//     Greater than 3: Excellent
//
// ref: [Sharpe Ratio](https://corporatefinanceinstitute.com/resources/career-map/sell-side/risk-management/sharpe-ratio-definition-formula/)
// 
//
/// ## Sharpe Ratio Formula
/// 
/// sharpe = (mean risk - risk free return) / standard_deviation of mean risk
///
/// arguments: 
///     `series: &[f64]` - returns over the span as vec of f64 // in decimal (not %)
///     `rf: f64` - risk free return // in decimal (not %)
pub fn calculate_sharpe(series: &[f64], rf: f64) -> f64 {
    let m = mean(series);
    (m - rf) / standard_deviation(variance(series, Some(m)))
}

// modified version of variance.
// We will only look at returns that fall below
// our threshold (in this case, our risk-free rate or zero).
// Downside Variance: only considers returns lower than the target (rf)
pub fn downside_deviation(series: &[f64], rf: f64) -> f64 {
    let count = series.len() as f64;
    let sum_sq_diffs: f64 = series
        .iter()
        .map(|&x| {
            let diff = x - rf;
            if diff < 0.0 { diff.powi(2) } else { 0.0 }
        })
        .sum();

    (sum_sq_diffs / count).sqrt()
}

pub fn calculate_sortino(series: &[f64], rf: f64) -> f64 {
    let m = mean(series);
    let d_dev = downside_deviation(series, rf);

    if d_dev == 0.0 {
        return 0.0;
    }

    (m - rf) / d_dev
}

// Treynor Ratio.

// Unlike Sharpe and Sortino (which look at the fund's internal wobbles),
// the Treynor Ratio compares the fund to the Market (Beta).
// It asks: "Is this fund risky because it's volatile, or is it risky because the whole market is moving?"
//
// The Treynor Ratio moves the goalposts.
// While Sharpe and Sortino look at total risk (standard deviation),
// the Treynor Ratio only cares about systematic risk—the risk that
// cannot be diversified away because it’s tied to the market as a whole.
//
// To calculate this, we swap out Standard Deviation for Beta (β).
//
/// ## The Treynor Formula
///
/// TreynorRatio = Rm − Rf / Beta
/// 
/// Where:
///     Rm: Portfolio Return.
///     Rf: Risk-free Rate.
///     Beta: Sensitivity of the portfolio to market movements.
///
/// Treynor = (mean return - risk free) / beta
pub fn calculate_treynor(series: &[f64], market: &[f64], rf: f64) -> f64 {
    let beta: f64 = Beta::new(series, market).into();

    (mean(series) - rf) / beta
}

// It measures a manager's ability to generate "Alpha" (excess return) relative to a benchmark, specifically looking at the "Tracking Error."
//
// The Information Ratio (IR)

// The Information Ratio is the "Report Card" for active fund managers.
// It doesn't just ask if you made money; it asks:
//      "Did you beat the benchmark, and was it skill or just luck?"
// 
// The Formula
// 
// InformationRatio = Tracking ErrorPortfolio Return − Benchmark Return​
//
// Where:
//
//     Active Return: The difference between your fund and the market (S&P 500).
//
//     Tracking Error: The standard deviation of that difference (how consistently you beat or trailed the market).
pub fn calculate_information_ratio(series: &[f64], market: &[f64]) -> f64 {
    let diffs: Vec<f64> = series
        .iter()
        .zip(market.iter())
        .map(|(&s, &m)| s - m)
        .collect();

    let active_return = mean(&diffs);
    let tracking_error = standard_deviation(variance(&diffs, Some(active_return)));

    if tracking_error == 0.0 {
        return 0.0;
    }
    active_return / tracking_error
}
