use corporate_finance::risk_metrics::*;

fn main() {
    let market = vec![0.10];
    let rf = 0.065;

    let portfolio = vec![
        ("Tech Growth", vec![0.15, 0.25, -0.05, 0.40, -0.10]),
        ("Stable Utility", vec![0.07, 0.08, 0.06, 0.09, 0.07]),
    ];

    // Covariance(series, market)
    let _co_variance = portfolio[0]
        .1
        .iter()
        .zip(market.iter())
        .map(|(&s, &m)| {
            println!("{s}, {m}");
            s + m
        })
        .sum::<f64>();

    for (name, series) in portfolio {
        let metrics = InvestmentMetrics {
            name: name.to_string(),
            sharpe: calculate_sharpe(&series, rf),
            sortino: calculate_sortino(&series, rf),
            treynor: calculate_treynor(&series, &market, rf),
            info_ratio: calculate_information_ratio(&series, &market),
            beta: Beta::new(&series, &market).into(),
        };
        metrics.print_full_analysis();
    }
}

fn report(name: &str, series: &[f64], market: &[f64], rf: f64) {
    let metrics = InvestmentMetrics {
        name: name.to_string(),
        sharpe: calculate_sharpe(series, rf),
        sortino: calculate_sortino(series, rf),
        treynor: calculate_treynor(series, market, rf),
        info_ratio: calculate_information_ratio(series, market),
        beta: Beta::new(series, market).into(),
    };

    // Print raw numbers
    println!(
        "{}: Sharpe: {:.2}, Sortino: {:.2}, Beta: {:.2}",
        metrics.name, metrics.sharpe, metrics.sortino, metrics.beta
    );

    // Print deep insights
    metrics.print_insight();
}

pub struct InvestmentMetrics {
    pub name: String,
    pub sharpe: f64,
    pub sortino: f64,
    pub treynor: f64,
    pub info_ratio: f64,
    pub beta: f64,
}

pub const LIST_MARKER: &str = "•";
pub const DASH_MARKER: &str = "-";
pub const BOLD_DASH_MARKER: &str = "━";

impl InvestmentMetrics {
    pub fn print_insight(&self) {
        println!("--- Financial Insights for {} ---", self.name);

        // Sharpe Insight
        if self.sharpe < 1.0 {
            println!(
                "• [Risk/Reward]: The Sharpe ratio ({:.2}) is low. You aren't being paid enough for the total volatility you're enduring.",
                self.sharpe
            );
        } else {
            println!(
                "• [Risk/Reward]: Strong Sharpe ratio. The fund offers efficient returns relative to its price swings."
            );
        }

        // Sortino Insight
        if self.sortino > self.sharpe * 1.5 {
            println!(
                "• [Volatility]: Note the high Sortino ({:.2}). This suggests the volatility is mostly 'upside'—the fund jumps up but rarely crashes hard.",
                self.sortino
            );
        }

        // Beta Insight
        if self.beta > 1.5 {
            println!(
                "• [Market]: High Beta ({:.2}). This fund is an 'amplifier'. It will soar in bull markets but likely bleed twice as fast in a crash.",
                self.beta
            );
        } else if self.beta < 0.5 {
            println!(
                "• [Market]: Low Beta ({:.2}). This is a defensive play. It provides 'uncorrelated' returns, acting as a hedge against market chaos.",
                self.beta
            );
        }

        // Treynor/IR Insight
        if self.info_ratio > 0.5 {
            println!(
                "• [Skill]: The Information Ratio ({:.2}) indicates a skillful manager. They are successfully 'beating the market' through active selection.",
                self.info_ratio
            );
        }

        println!("{}\n", DASH_MARKER.repeat(50));
    }

    pub fn get_recommendation(&self) -> (&str, &str) {
        if self.sortino > 3.0 && self.beta < 0.5 {
            (
                "CORE HOLDING",
                "Low risk, extreme downside protection. Suitable for 20-40% of portfolio.",
            )
        } else if self.info_ratio > 0.5 && self.beta > 1.5 {
            (
                "TACTICAL GROWTH",
                "High octane. Manager shows skill, but market sensitivity is high. Limit to 5-10%.",
            )
        } else if self.sharpe < 1.0 && self.info_ratio < 0.2 {
            (
                "AVOID / REDUCE",
                "Poor risk-adjusted returns and little evidence of manager skill.",
            )
        } else {
            (
                "SPECULATIVE",
                "Inconsistent metrics. Use only for small, non-core positions.",
            )
        }
    }

    pub fn print_full_analysis(&self) {
        let (rec_title, rec_desc) = self.get_recommendation();

        println!("{}", BOLD_DASH_MARKER.repeat(50));
        println!("ANALYSIS FOR: {}", self.name.to_uppercase());
        println!("{}", BOLD_DASH_MARKER.repeat(50));

        self.print_insight(); // Your previous insight logic

        println!("PROPOSED ACTION: **{}**", rec_title);
        println!("RATIONALE: {}", rec_desc);
        println!("{}", BOLD_DASH_MARKER.repeat(50));
    }
}
