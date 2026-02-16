use crate::{covariance, mean, sd, variance};

pub struct Beta(f64);

// # Beta
//
// ref: https://corporatefinanceinstitute.com/resources/valuation/what-is-beta-guide/
//
// Symbol: β
//
// The beta coefficient can be interpreted as follows:
//
//     β = 1: exactly as volatile as the market
//     β > 1: more volatile than the market (higher risk and potential return)
//     β < 1 (but > 0): less volatile than the market
//     β = 0: uncorrelated to the market
//     β < 0: negatively correlated to the market
//
// ## Formula
//
// Beta = Covariance (Re, Rm) / Variance (Rm)
//
impl Beta {
    pub fn new(series: &[f64], market: &[f64]) -> Self {
        // If they don't match, the math is technically invalid for a specific timeframe
        assert_eq!(
            series.len(),
            market.len(),
            "series[{}] and market[{}] must have the same mumber of data points",
            series.len(),
            market.len()
        );

        if series.is_empty() {
            return Self(0.0);
        }

        let beta = covariance!(series, market) / variance!(market);
        Self(beta)
    }

    // if Beta > 1.0 => The fund is more volatile than the market
    /// If true, it means the fund moves exactly with the market.
    pub const fn is_one(&self) -> bool {
        // self.0 == 1.00 - this is risky due to precision errors
        // check if the difference is within a tiny margin (epsilon)
        (self.0 - 1.0).abs() < 1e-6 // or f64::EPSILON? isn't EPSILON way too small for
                                    // financial calculations
    }

    /// The fund is "defensive" and moves less than the market
    pub const fn is_negative(&self) -> bool {
        self.0.is_sign_negative()
    }

    pub const fn is_positive(&self) -> bool {
        self.0.is_sign_positive()
    }

    pub const fn value(&self) -> f64 {
        self.0
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

pub fn sharpe(series: &[f64], rf: f64) -> f64 {
    internal_sharpe(Some(series), rf, None, None)
}

fn internal_sharpe(series: Option<&[f64]>, rf: f64, rp: Option<f64>, sd: Option<f64>) -> f64 {
    let portfolio_ret: f64;
    let std_div: f64;
    if let Some(series) = series {
        portfolio_ret = mean!(series);
        std_div = sd!(variance!(series, portfolio_ret));
    } else {
        assert!(rp.is_some(), "");
        assert!(sd.is_some(), "");
        portfolio_ret = rp.unwrap();
        std_div = sd.unwrap();
    }

    if std_div < f64::EPSILON {
        return 0.0;
    }

    (portfolio_ret - rf) / std_div
}

/// 1. sharpe!(series, rf);
/// 2. sharpe!(rp, rf, sd);
#[macro_export]
macro_rules! sharpe {
    ($series: expr, $rf: expr) => {
        $crate::ratios::risk_metrics::internal_sharpe(Some($series), $rf, None, None)
    };
    ($rp: expr, $rf: expr, $sd: expr) => {
        $crate::ratios::risk_metrics::internal_sharpe(None, $rf, Some($rp), Some($sd))
    };
}

#[cfg(test)]
mod test {
    use crate::ratios::risk_metrics::sharpe;

    use super::Beta;
    
    // in the month of Jan 2026
    const NIFTY_50: [f64; 19] = [
        0.006960765170238605,
        -0.002972058760474052,
        -0.002727647317136396,
        -0.0014496220164682432,
        -0.01009536415844993,
        -0.007479613285493556,
        0.004164153963733427,
        -0.0022469428853927357,
        -0.0025921184600641006,
        0.0011201764399651254,
        -0.0042363247573810724,
        -0.013796877137441129,
        -0.002972357079163777,
        0.0052628596094604,
        -0.009539381186706124,
        0.005060152863462813,
        0.006647346488174181,
        0.003004819548983437,
        -0.003865234077404724,
    ];
    // in the month of Jan 2026
    const ITC: [f64; 19] = [
        -0.03792780822846855,
        -0.0009997348459663343,
        -0.02073202469727801,
        -0.0035042128799998833,
        -0.001025698054907989,
        -0.011000239733699091,
        0.003707530300019869,
        -0.0109337303722129,
        0.000149316018136497,
        -0.016579515057863883,
        0.012150627231730476,

        -0.02070828665292772,
        -0.00475024862225316,
        0.00030797372763436743,
        -0.004463668706180607,
        -0.014687699127850123,
        0.007845658409425468,
        -0.007940199219514013,
        0.011142447553835816,
    ];
    #[test]
    fn beta_t() {

        let beta: f64 = Beta::new(&ITC, &NIFTY_50).into();
        assert_eq!(beta, -0.13098715705340794);
    }

    #[test]
    fn sharpe_t() {
        let protfolio_return = 0.18; // one year return = 18%

        let rf = 0.03; // risk free return = 3%
        let annaulized_sd = 0.12;

        let s1 = sharpe!(&ITC, rf);
        let s3 = sharpe(&ITC, rf);
        let s2 = sharpe!(protfolio_return, rf, annaulized_sd);
        assert_eq!(s2, 1.25);
        assert_eq!(s1, -3.024907069875915);
        assert_eq!(s3, -3.024907069875915);
    }
}
